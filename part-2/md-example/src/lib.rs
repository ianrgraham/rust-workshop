// TODO there should also be a mechanism by which bonded pairs can be expressed
// this may be possible with the way that we have defined our vec of dyn
// potentials

// I think I've learned a few important lessons about traits in trying to do
// this. Traits are quite a powerful feature for building interfaces, but there
// are some tricks to getting it to work for you

use ndarray::prelude::*;
use ndarray::Zip;
use std::fmt::Result;
use std::fs::OpenOptions;
use std::io;
use std::io::{BufWriter, Write};

// Simulation state
pub struct Simulation {
    timestep: usize,
    configuration: Configuration,
    pub integrator: Option<Box<dyn Integrator>>,
    pub nlist: Option<Box<dyn NeighborList>>,
    pub potential: Option<Box<dyn Potential>>,
    pub writer: Option<Box<dyn Writer>>,
}

// Implementation block for simulation
impl Simulation {
    /// Run the simulation
    pub fn run(&mut self, steps: usize) {
        let pos_dim = self.configuration.position.raw_dim();
        let num = pos_dim[0];

        // Unwrap optional data structures
        let nlist = self
            .nlist
            .as_mut()
            .expect("No neighborlist has been specified!");
        let pot = self
            .potential
            .as_ref()
            .expect("No potential has been specified!");
        let integrator = self
            .integrator
            .as_ref()
            .expect("No integration method has been specified!");

        for _ in 0..steps {
            if let Some(writer) = &mut self.writer {
                if writer.trigger(self.timestep) {
                    writer.write(&self.configuration);
                }
            }

            nlist.query(&self.configuration);

            let (pairs, force_buffer) = pot.build_force_buffer(&self.configuration, nlist.as_mut());

            let mut force = Array2::<f32>::zeros(pos_dim);
            for (pair, force_comp) in pairs.into_iter().zip(force_buffer) {}

            integrator.update_configuration(&mut self.configuration, force.view());

            self.timestep += 1;
        }
    }
}

pub struct Configuration {
    position: Array2<f32>,
    velocity: Array2<f32>,
    diameter: Array1<f32>,
    type_ids: Array1<u8>,
    box_dim: [f32; 3],
    box2_dim: [f32; 3],
    dim: u8,
}

impl Configuration {
    /// Validate a prospective simulation config
    pub fn new(
        position: Array2<f32>,
        velocity: Array2<f32>,
        diameter: Array1<f32>,
        type_ids: Array1<u8>,
        box_dim: [f32; 3],
        dim: u8,
    ) -> Self {
        let pos_shape = position.raw_dim();
        let vel_shape = velocity.raw_dim();
        assert_eq!(pos_shape[1], 3);
        assert_eq!(pos_shape, vel_shape);
        assert_eq!(pos_shape[0], diameter.len());
        assert_eq!(pos_shape[0], type_ids.len());
        assert!(dim == 2 || dim == 3);
        assert!(box_dim.iter().all(|x| x.is_sign_positive()));
        let mut box2_dim = [0.0f32; 3];
        box2_dim
            .iter_mut()
            .zip(box_dim)
            .for_each(|(b2, b)| *b2 = b / 2.0);
        Configuration {
            position,
            velocity,
            diameter,
            type_ids,
            box_dim,
            box2_dim,
            dim,
        }
    }

    fn pbc_vec(&self, i: u32, j: u32) -> [f32; 3] {
        let mut vec = [0.0f32; 3];

        {
            // scope to do mut Zip on NdProducer of underlying slice
            let mut vec = ArrayViewMut1::from(&mut vec[..self.dim as usize]);
            Zip::from(&self.position.row(i as usize))
                .and(&self.position.row(j as usize))
                .and(&mut vec)
                .for_each(|xi, xj, v| *v = xi - xj);
        }

        // apply PBCs
        vec.iter_mut()
            .take(self.dim as usize)
            .zip(self.box2_dim)
            .zip(self.box_dim)
            .for_each(|((v, b2), b)| {
                if *v > b2 {
                    *v -= b
                } else if *v <= -b2 {
                    *v += b
                }
            });

        vec
    }

    fn norm_squared(&self, vec: &[f32; 3]) -> f32 {
        let x = vec
            .iter()
            .take(self.dim as usize)
            .fold(0.0, |acc, &x| acc + x * x);
        x
    }
}

// takes information of simulation state and updates particles
// what other bits of data should be defined for this?
pub trait Integrator {
    fn update_configuration(&self, config: &mut Configuration, force: ArrayView2<f32>) {
        Zip::from(config.position.rows_mut())
            .and(config.velocity.rows_mut())
            .and(force.rows())
            .for_each(|pos, vel, force| self.update_pos(pos, vel, force));
    }

    fn update_pos(&self, pos: ArrayViewMut1<f32>, vel: ArrayViewMut1<f32>, force: ArrayView1<f32>);
}

struct OverdampedLangevin {
    dt: f32,
    beta: f32,
    a: f32,
    b: f32,
}

impl OverdampedLangevin {
    fn new(dt: f32, beta: f32, eta: f32) -> Self {
        todo!()
    }
}

// takes information of simulation state and computes forces
pub trait Potential {
    fn build_force_buffer(
        &self,
        config: &Configuration,
        nlist: &mut dyn NeighborList,
    ) -> (Vec<Pair>, Vec<[f32; 3]>) {
        let mut pairs = Vec::<Pair>::new();
        let mut forces = Vec::<[f32; 3]>::new();
        nlist.for_each(|(i, j)| {
            let mut dv = config.pbc_vec(i, j);
            let rsqr = config.norm_squared(&dv);

            if let Some(force_divr) = self.force_divr(
                rsqr,
                config.type_ids[i as usize],
                config.type_ids[j as usize],
            ) {
                pairs.push((i, j));
                dv.iter_mut()
                    .take(config.dim as usize)
                    .for_each(|x| *x *= force_divr);
                forces.push(dv)
            }
        });

        return (pairs, forces);
    }

    fn force_divr(&self, invr2: f32, type_i: u8, type_j: u8) -> Option<f32>;

    fn max_cutoff(&self) -> f32;
}

struct BiLjPotential {
    lj1: [f32; 3],
    lj2: [f32; 3],
    cut_sqr: [f32; 3],
}

impl Default for BiLjPotential {
    fn default() -> Self {
        let aa: (f32, f32) = (1.0, 1.0);
        let ab: (f32, f32) = (1.5, 0.8);
        let bb: (f32, f32) = (0.5, 0.88);

        let lj1 = |(epsilon, sigma): (f32, f32)| 48.0 * epsilon * sigma.powi(12);
        let lj2 = |(epsilon, sigma): (f32, f32)| -24.0 * epsilon * sigma.powi(6);
        let cut_sqr = |(_, sigma): (f32, f32)| (2.5 * sigma).powi(2);

        BiLjPotential {
            lj1: [lj1(aa), lj1(ab), lj1(bb)],
            lj2: [lj2(aa), lj2(ab), lj2(bb)],
            cut_sqr: [cut_sqr(aa), cut_sqr(ab), cut_sqr(bb)],
        }
    }
}

impl Potential for BiLjPotential {
    fn force_divr(&self, rsqr: f32, type_i: u8, type_j: u8) -> Option<f32> {
        let idx = (type_i + type_j) as usize;
        if rsqr < self.cut_sqr[idx] {
            let lj1 = self.lj1[idx];
            let lj2 = self.lj2[idx];
            let r2inv = 1.0 / rsqr;
            let r6inv = r2inv * r2inv * r2inv;
            let force_divr = r2inv * r6inv * (12.0 * lj1 * r6inv - 6.0 * lj2);
            Some(force_divr)
        } else {
            None
        }
    }

    fn max_cutoff(&self) -> f32 {
        self.cut_sqr.into_iter().reduce(f32::max).unwrap().sqrt()
    }
}

type Pair = (u32, u32);

// checks that particles are still in their boxes, updates box if necessary
pub trait NeighborList: Iterator<Item = Pair> {
    /// At the beginning of a frame, setup data to be iterated over
    fn query(&mut self, config: &Configuration);
}

#[derive(Default)]
struct NaiveList {
    i: u32,
    j: u32,
    n: u32,
}

impl NeighborList for NaiveList {
    fn query(&mut self, config: &Configuration) {
        self.i = 0;
        self.j = 0;

        let n = config.position.raw_dim()[0];
        self.n = n as u32 - 1;
    }
}

impl Iterator for NaiveList {
    type Item = Pair;

    fn next(&mut self) -> Option<Self::Item> {
        if self.j < self.n {
            self.j += 1;
            Some((self.i, self.j))
        } else if self.i < self.n - 1 {
            self.i += 1;
            self.j = self.i + 1;
            Some((self.i, self.j))
        } else {
            None
        }
    }
}

// struct CellList {
//     cells: Array3<Vec<u32>>,
//     stride: [f32; 3]
// }

// impl NeighborList for CellList {}

pub trait Writer {
    fn trigger(&self, timestep: usize) -> bool;
    fn write(&mut self, config: &Configuration) -> anyhow::Result<()>;
}

// Writes MD output to the XYZ format, readable by Ovito
pub struct XyzWriter {
    file_handle: BufWriter<std::fs::File>,
    period: usize,
    phase: usize,
}

impl XyzWriter {
    fn new(path: &str, period: usize, phase: usize) -> Self {
        let file_handle = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)
            .unwrap();
        let file_handle = BufWriter::new(file_handle);
        XyzWriter {
            file_handle,
            period,
            phase,
        }
    }
}

use ndarray::FoldWhile::*;

impl Writer for XyzWriter {
    fn trigger(&self, timestep: usize) -> bool {
        (timestep - self.phase) % self.period == 0
    }

    fn write(&mut self, config: &Configuration) -> anyhow::Result<()> {
        let pos = config.position.view();
        let diams = config.diameter.view();
        let type_ids = config.type_ids.view();

        let file = &mut self.file_handle;

        writeln!(file, "{}\n", pos.len())?;

        Zip::from(type_ids).and(pos.rows()).and(diams).fold_while(
            Ok(()),
            |_, id, x, d| {
                let res = writeln!(file, "{} {} {} {} {}", id, x[0], x[1], x[2], d);
                if res.is_ok() {
                    Continue(res)
                }
                else {
                    Done(res)
                }
            }
        ).into_inner()?;

        Ok(())
    }
}

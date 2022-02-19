
// TODO there should also be a mechanism by which bonded pairs can be expressed
// this may be possible with the way that we have defined our vec of dyn potentials

// I think I've learned a few important lessons about traits in trying to do this.
// Traits are quite a powerful feature for building interfaces, but there are some
// tricks to getting it to work for you

use ndarray::prelude::*;
use ndarray::Zip;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};


// Simulation state
pub struct Simulation {
    timestep: usize,
    configuration: Configuration,
    pub integrator: Option<Box<dyn Integrator>>,
    pub nlist: Option<Box<dyn NeighborList>>,
    pub potential: Option<Box<dyn Potential>>,
    pub writer: Option<Box<dyn Writer>>
}

// Implementation block for simulation
impl Simulation {

    /// Run the simulation
    pub fn run(&mut self, steps: usize) {

        let pos_dim = self.configuration.position.raw_dim();
        let num = pos_dim[0];

        // Unwrap optional data structures
        let nlist = self.nlist.expect("No neighborlist has been specified!");
        let pot = self.potential.expect("No potential has been specified!");
        let integrator = self.integrator.expect("No integration method has been specified!");


        for _ in 0..steps {

            if let Some(writer) = self.writer {
                if writer.trigger(self.timestep) {

                }
            }

            nlist.query(&self.configuration);

            let (pairs, force_buffer) = pot.build_force_buffer(&self.configuration, nlist.as_ref());
            
            let force = Array2::<f32>::zeros(pos_dim);
            for (pair, force_comp) in pair.iter().zip(force) {

            }

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
    dim: u8
}

impl Configuration {

    /// Validate a prospective simulation config
    pub fn new(
        position: Array2<f32>,
        velocity: Array2<f32>,
        diameter: Array1<f32>,
        type_ids: Array1<u8>,
        box_dim: [f32; 3],
        dim: u8
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
        box2_dim.iter_mut().zip(box_dim).for_each(|(b2, b)| *b2 = b/2.0);
        Configuration{
            position,
            velocity,
            diameter,
            type_ids,
            box_dim,
            box2_dim,
            dim
        }
    }
    
    fn pbc_vec(&self, i: u32, j: u32) -> [f32; 3] {

        let mut vec = [0.0f32; 3];

        { // scope to do mut Zip on NdProducer of underlying slice
            let mut vec = ArrayViewMut1::from(&mut vec[..self.dim as usize]);
            Zip::from(&self.position.row(i as usize))
                .and(&self.position.row(j as usize))
                .and(&mut vec)
                .for_each(
                    |xi, xj, v| *v = xi - xj
                );
        }

        // apply PBCs
        vec.iter_mut().take(self.dim as usize).zip(self.box2_dim).zip(self.box_dim).for_each(
            |((v, b2), b)| {
                if *v > b2 {*v -= b}
                else if *v <= -b2 {*v += b}
            }
        );

        vec
    }

    fn norm_squared(&self, vec: &[f32; 3]) -> f32 {
        let x = vec.iter()
            .take(self.dim as usize)
            .fold(0.0, |acc, &x| acc + x*x);
        x
    }

}

// takes information of simulation state and updates particles
// what other bits of data should be defined for this?
trait Integrator {
    fn update_configuration(&self, config: &mut Configuration, force: ArrayView2<f32>) {

        Zip::from(config.position.rows_mut())
            .and(config.velocity.rows_mut())
            .and(force.rows())
            .for_each(|pos, vel, force| self.update_pos(pos, vel, force));
    }

    fn update_pos(
        &self,
        pos: ArrayViewMut1<f32>,
        vel: ArrayViewMut1<f32>,
        force: ArrayView1<f32>
    );
}

// takes information of simulation state and computes forces
trait Potential {
    fn build_force_buffer(&self, config: &Configuration, nlist: &dyn NeighborList) {
        for ((i, j), f) in nlist.into_iter() {
            let dv = config.pbc_vec(*i, *j);
            let r2inv = config.norm_squared(&dv);

            let force = self.force_divr(r2inv, config.type_ids[*i as usize], config.type_ids[*i as usize]);
        }
    }

    fn force_divr(&self, invr2: f32, type_i: u8, type_j: u8) -> Option<f32>;

    fn max_cutoff(&self) -> f32;
}

struct BiLjPotential {
    lj1: [f32; 3],
    lj2: [f32; 3],
    cut_sqr: [f32; 3]
}

impl Default for BiLjPotential {
    fn default() -> Self {
        let aa: (f32, f32) = (1.0, 1.0);
        let ab: (f32, f32) = (1.5, 0.8);
        let bb: (f32, f32) = (0.5, 0.88);

        let lj1 = |(epsilon, sigma): (f32, f32)| 48.0*epsilon*sigma.powi(12);
        let lj2 = |(epsilon, sigma): (f32, f32)| -24.0*epsilon*sigma.powi(6);
        let cut_sqr = |(_, sigma): (f32, f32)| (2.5*sigma).powi(2);

        BiLjPotential{
            lj1: [lj1(aa), lj1(ab), lj1(bb)],
            lj2: [lj2(aa), lj2(ab), lj2(bb)],
            cut_sqr: [cut_sqr(aa), cut_sqr(ab), cut_sqr(bb)]
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
        }
        else {
            None
        }
    }

    fn max_cutoff(&self) -> f32 {
        self.cut_sqr.into_iter().reduce(f32::max).unwrap().sqrt()
    }
}

type Pair = (u32, u32);
type NeighborListQuery = [Pair];


// checks that particles are still in their boxes, updates box if necessary
trait NeighborList {
    fn query(&mut self, config: &Configuration);
}

impl Iterator for &dyn NeighborList {
    type Item = Pair;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

// struct NeighborListIter<'a, NList: NeighborList> {
//     nlist: &'a NList
// }

// impl<'a, NList: NeighborList> Iterator for NeighborListIter<'a, NList> {
//     type Item = Pair;

//     fn next(&mut self) -> Option<Pair> {
//         self.nlist.next_pair()
//     }
// }

struct NaiveList {}



// struct CellList {
//     cells: Array3<Vec<u32>>,
//     stride: [f32; 3]
// }

// impl NeighborList for CellList {}

trait Writer {
    fn trigger(&self, timestep: usize) -> bool;
    fn write(&self, config: &Configuration) -> Result<(), anyhow::Error>;
}

pub struct XyzWriter {
    file_handle: BufWriter<std::fs::File>,
    period: usize,
    phase: usize
}

impl XyzWriter {
    fn new(path: &str, period: usize, phase: usize) -> Self{
        let file_handle = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)
            .unwrap();
        let file_handle = BufWriter::new(file_handle);
        XyzWriter{
            file_handle,
            period,
            phase
        }
    }
}

impl Writer for XyzWriter {
    fn trigger(&self, timestep: usize) -> bool {
        (timestep - self.phase) % self.period == 0
    }

    fn write(&slef, config: &Configuration) -> Result<(), anyhow::Error> {
        let pos = config.position;

        let sigmas = match self.sys.potential {
            Hertz => { &self.sys.sigmas[..] },
            LJ | WCA => { &[0.4, 0.5] }
        };
    
        writeln!(file, "{}\n", pos.len()).expect("FILE IO ERROR!");
        
        for (x, typeid) in pos.rows().zip(&self.sys.types) {
            writeln!(
                file, 
                "{} {} {} {} {}", 
                if *typeid == 0 { "A" } else { "B" },
                x[0], 
                x[1], 
                x[2],
                sigmas[*typeid]
            ).expect("FILE IO ERROR!");
        }
    }
}
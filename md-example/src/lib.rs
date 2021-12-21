
// TODO there should also be a mechinism by which bonded pairs can be expressed
// this may be possible with the way that we have defined our vec of dyn potentials

// I think I've learned a few important lessons about traits in trying to do this.
// Traits are quite a powerful feature for building interfaces, but there are some
// tricks to getting it to work for you

use ndarray::prelude::*;
use ndarray::Zip;


// Simulation state
#[derive(Default)]
pub struct Simulation {
    timestep: usize,
    configuration: Configuration,
    integrator: Option<Box<dyn Integrator>>,
    nlist: Option<Box<dyn NeighborList>>,
    potentials: Vec<Box<dyn Potential>>,
    workhorse: SimWorkhorse
}

// Implementation block for simulation
impl Simulation {

    /// Run the simulation
    pub fn run(&mut self, steps: usize) {
        for _ in 0..steps {
            self.query_nlist_and_clear_force_buffer();

            self.update_force_buffer();

            self.apply_buffer_to_forces();

            self.integrate_dynamics();
        }
    }

    fn query_nlist_and_clear_force_buffer(&mut self) {
        match &mut self.nlist {
            Some(nlist) => {
                let nlist = nlist.query(&self.configuration);
                self.workhorse.nlist_query = nlist.to_vec(); // this is a spurious copy, should remove somehow
                let target_len = nlist.len();
                self.workhorse.nlist_force_buffer.clear();
                self.workhorse.nlist_force_buffer.resize(target_len, [0.0, 0.0, 0.0]);
            },
            None => panic!("No neighborlist has been specified")
        }
    }

    fn update_force_buffer(&mut self) {
        assert_ne!(self.potentials.len(), 0);
        for pot in self.potentials.iter() {
            pot.update_to_force_buffer(&mut self.configuration, &mut self.workhorse);
        }
    }

    fn apply_buffer_to_forces(&mut self) {
        let wh = &self.workhorse;
        let sim_forces = &mut self.configuration.force;
        for (force, (i, j)) in wh.nlist_force_buffer.iter().zip(&wh.nlist_query) {
            let mut forces_slice = sim_forces.slice_mut(s![*i as usize,..]);
            let new_force = ArrayView::from(&force[..self.configuration.dim as usize]);
            forces_slice += &new_force;
        }
    }

    fn integrate_dynamics(&mut self) {
        match &self.integrator {
            Some(integrator) => integrator.update_configuration(&mut self.configuration),
            None => panic!("No integrator as been specified")
        }
    }
}

#[derive(Default)]
struct Configuration {
    position: Array2<f32>,
    velocity: Array2<f32>,
    force: Array2<f32>,
    type_ids: Array1<u8>,
    box_dim: [f32; 6],
    dim: u8,
    types: Vec<u8>
}

#[derive(Default)]
struct SimWorkhorse {
    nlist_force_buffer: Vec<[f32;3]>,
    nlist_query: Vec<Pair>
}

// takes information of simulation state and updates particles
// what other bits of data should be defined for this?
trait Integrator {
    fn update_configuration(&self, config: &mut Configuration) {

        Zip::from(config.position.rows_mut())
            .and(config.velocity.rows_mut())
            .and(config.force.rows())
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
    fn update_to_force_buffer(&self, config: &mut Configuration, workhorse: &mut SimWorkhorse) {

    }

    fn force(&self, dr: f32, type_i: u8, type_j: u8) -> f32;
}

struct LJPotential {
    lj1: f32,
    lj2: f32
}

impl Potential for LJPotential {

    fn force(&self, dr: f32, type_i: u8, type_j: u8) -> f32 {
        0.0
    }
}

type Pair = (u32, u32);
type NeighborListQuery = [Pair];

// checks that particles are still in their boxes, updates box if necessary
trait NeighborList: Component {
    fn query(&mut self, sim: &Configuration) -> &NeighborListQuery;
}

//
struct CellList {
    cells: ArrayD<Vec<u32>>,
    nlist: Vec<Pair>,
    stride: Box<[f64]>
}

impl Component for CellList { }

impl NeighborList for CellList {
    
    fn query(&mut self, config: &Configuration) -> &NeighborListQuery {
        // check that particles are still in their cells
        &self.nlist[..]
    }
}

trait Component {
}
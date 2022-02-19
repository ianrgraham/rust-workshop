/// This executable is a simple example of how to building an MD code in Rust.
/// Here we prioritize readability and conciseness above all else. MD codes in 
/// the wild may use more sophisticated methods to achieve the same results.
/// While these are all possible in Rust, we want to give a picture of how
/// Rust

use std::io::Write
use std::fs::File
use std::HashMap;

fn main() {
    
    let n = 8_usize.pow(3);
    let get_cubic_box = |num: usize, phi: f32| {
        let l = (num as f32 / phi).powf(1.0/3.0);
        [l, l, l, 0.0, 0.0, 0.0]
    };


    let snapshot = Snapshot{
        pos: vec![[0.0, 0.0, 0.0]; n],
        vel: vec![[0.0, 0.0, 0.0]; n],
        diam: vec![0.0; n],
        typeids: vec![0; n],
        types: 1,
        bbox: get_cubic_box(n, 1.2)
    };

    let harm = Box::new(Harmonic::bidisperse_std());
    let langevin = Box::new(Langevin{
        
    });
    let cell_list = Box::new(NaiveList{

    });

    let mut sim = Simulation::new(
        snapshot,
        harm,
        langevin,
        cell_list
    );

    sim.potentials
}

struct Simulation {
    snapshot: Snapshot,
    potential: Box<dyn Potential>,
    integration: Box<dyn Integration>,
    nlist: Box<dyn NeighborList>
}

struct Snapshot {
    pos: Vec<[f32; 3]>,
    vel: Vec<[f32; 3]>,
    diam: Vec<f32>,
    typeids: Vec<u8>,
    types: u8,
    bbox: [f32; 6]
}

type ForceVec = Vec<Option<[f32; 3]>>;

trait Potential {

    fn system_forces(&self, snapshot: &Snapshot, nlist_query: &NeighborlistQuery) -> ForceVec {
        for (i, j) in nlist_query {
            let x_i = snapshot.x[i]
            let x_j = snapshot.x[j]
        }
    }

    fn pair_force(&self, invr2: f32, type_i: u8, type_j: u8) -> f32;
}

trait Integration {

    fn update_system(&self, snapshot: &mut Snapshot, forces: ForceVec) {
        for option_force in forces {
            if let Some(force) = 
        }
    }

    fn update_particle(&self, pos: &mut [f32; 3], vel: &mut [f32; 3], force: &[f32; 3], typeid: u8);
}

type NeighborlistQuery = Vec<(u32, u32)>;

trait NeighborList {

    fn query_neighbors(&self, snapshot: &Snapshot) -> NeighborlistQuery;

}

impl Simulation {

    fn new(
        snapshot: Snapshot,
        potential: Box<dyn Potential>,
        integration: Box<dyn Integration>,
        nlist: Box<dyn NeighborList>
    ) -> Self {
        todo!();
        // verify the snapshot is a valid system before plugging it in
        // checks at the beginning of a simulation are one way to avoid 
        // bugs down the road.
        Simulation {
            snapshot,
            potential,
            integration, 
            nlist
        }
    }

    fn run(&mut self, timesteps: usize) {
        
        for _ in 0..timesteps {
            // get nlist query
            let nlist_query = self.nlist.query_neighbors(&self.snapshot);

            // calculate forces
            let forces = self.potential.system_forces(&self.snapshot)

            // update positions
            self.integration.update_system(&mut self.snapshot, &)
        }
    }


}

stuct Harmonic {
    cutoffs: HashMap<(u8, u8), f32>

}

impl Potential for Harmonic {

    fn pair_force(&self, invr2: f32, type_i: u8, type_j: u8) -> f32 {
        
    }

}

struct Langevin {
    dt: f32,

}

impl Integration for Langevin {

}

struct NaiveList {}

impl NeighborList for NaiveList {
    
    fn query_neighbors(&self, snapshot: &Snapshot) -> NeighborlistQuery {
        let n = snapshot.x.size();
        let nlist = Vec::with_capacity(n*(n-1)/2);
        for i in 0..(n-1) {
            for j in i..n {
                nlist.push((i, j));
            }
        }
        nlist
    }

}

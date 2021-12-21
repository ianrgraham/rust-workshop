use parking_lot::Mutex;
// use std::sync::Mutex;
use std::sync::Arc;
use rand::prelude::*;
use rand::Fill;

#[macro_use]
extern crate timeit;

fn main() {

    let mut rng = thread_rng();
    let mut nums = [0.0f32; 100];
    nums.try_fill(&mut rng).unwrap();

    println!("{:?}", nums);

    timeit!({
        let mut my_vec = Vec::<Arc<Mutex<f32>>>::new();
        for num in &nums {
            my_vec.push(Arc::from(Mutex::from(*num)));
        }
        // println!("{:?}", my_vec);
    });

    timeit!({
        let mut my_vec = Vec::<Mutex<&f32>>::new();
        for num in &nums {
            my_vec.push(Mutex::from(num));
        }
        // println!("{:?}", my_vec);
    });

    timeit!({
        let mut my_vec = Vec::<f32>::new();
        my_vec.extend(&nums);
        // println!("{:?}", my_vec);
    });

    let mut my_vec = Vec::<Mutex<&f32>>::new();
    for num in &nums {
        my_vec.push(Mutex::from(num));
    }
    
    let mut tmp = 0.0;

    timeit!({
        tmp = nums[15];

    });

    println!("{}", tmp);

    let data = **my_vec[15].lock();
    timeit!({
        
        tmp = data;
    });
    println!("{}", tmp);
}

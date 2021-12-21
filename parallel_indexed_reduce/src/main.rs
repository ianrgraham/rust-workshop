use parking_lot::Mutex;
use std::sync::Arc;
use rand::prelude::*;
use rand::Fill;
use rayon::prelude::*;
use smallvec::SmallVec;

#[macro_use]
extern crate timeit;

const N: usize = 100000;
const M: usize = 100;

macro_rules! print_micro{
    ($sec:expr) => {
        println!("{:.2} \u{00B5}s", $sec/1e-6);
    }
}

#[inline(never)]
fn slow() {

    let mut rng = thread_rng();

    let mut output = vec![0.0f32; u8::MAX as usize + 1];

    let mut pairs = Vec::<SmallVec::<[(f32, u8);128]>>::new();
    for _ in 0..N/M {
        let mut p = SmallVec::<[(f32, u8);128]>::new();
        for _ in 0..M {
            p.push(rng.gen());
        }
        pairs.push(p);
    }

    let sec = timeit_loops!(1,{
        pairs.par_iter().zip(&mut output).for_each(|(pair, out)|{
            for p in pair {
                *out += p.0;
                // output[p.1 as usize] -= p.0;
            }
        })
    });

    print_micro!(sec);
}

#[inline(never)]
fn fast() {

    let mut rng = thread_rng();

    let mut output = vec![0.0f32; u8::MAX as usize + 1];

    let mut pairs = Vec::<(u8, u8)>::new();
    let mut inputs = Vec::<f32>::new();
    for _ in 0..N {
        pairs.push(rng.gen());
        inputs.push(rng.gen());
    }

    let sec = timeit_loops!(1000,{
        pairs.iter().zip(&inputs).for_each(|(pair, input)|{
            output[pair.0 as usize] += input;
            // output[pair.1 as usize] -= input;
        })
    });

    print_micro!(sec);
    println!("{:?}", output);
}

fn main() {

    // slow();

    fast();
    

    // let mut output = vec![0.0f32; u8::MAX as usize + 1];

    // let mut pairs = Vec::<SmallVec::<[(f32, u8);128]>>::new();
    // for _ in 0..N/M {
    //     let mut p = SmallVec::<[(f32, u8);128]>::new();
    //     for _ in 0..M {
    //         p.push(rng.gen());
    //     }
    //     pairs.push(p);
    // }

    // let sec = timeit_loops!(1,{
    //     pairs.par_iter().zip(&mut output).for_each(|(pair, out)|{
    //         for p in pair {
    //             *out += p.0;
    //             // output[p.1 as usize] -= p.0;
    //         }
    //     })
    // });

    // print_micro!(sec);



    // this is absurdely slow!
    // par_iter makes it worse!
    // let output = vec![Arc::new(Mutex::new(0.0f32)); u8::MAX as usize + 1];

    // let sec = timeit_loops!(100,{
    //     pairs.iter().zip(&inputs).for_each(|(pair, input)|{
    //         // println!("hello {:?}", pair);
    //         let mut data1 = output[pair.0 as usize].lock();
    //         *data1 += input;
    //         drop(data1);
    //         // println!("half done {:?}", pair);
    //         // let mut data2 = output[pair.1 as usize].lock();
    //         // *data2 -= input;
    //         // drop(data2);
    //         // println!("done {:?}", pair);
    //     })
    // });

    // print_micro!(sec);

    // println!("{:?}", output);

}

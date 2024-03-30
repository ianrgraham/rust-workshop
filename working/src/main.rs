use std::thread;
use std::collections;

fn main() {
    let a = 5;
    thread::scope(|s| {
        let t = thread::spawn(|| {println!("{}", &a)});
        t.join();
        let t = thread::spawn(|| {println!("Hi!")});
        t.join();
    });
}
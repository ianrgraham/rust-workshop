use std::ops::Add;

fn main() {
    let result = add(1, 2);
    println!("{}", result);
}

fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
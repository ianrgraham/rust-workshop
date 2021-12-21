fn main() {
    println!("Hello, world!");

    let a = sign(2);
    println!("The statement 2 >= 0 is {}", a);
}

fn sign(num: i32) -> bool {
    if num >= 0 { 
        true
    }
    else {
        false
    }
}

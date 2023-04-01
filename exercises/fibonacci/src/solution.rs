
// EXCERSISE: Write a function that returns the nth fibonacci number
fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    
    // Assert statement to check for our programs accuracy
    assert_eq!(fibonacci(36), 14930352);

    // EXERCISE! Print out the first 10 fibonacci numbers
    for i in 0..10 {
        println!("{}", fibonacci(i));
    }
}

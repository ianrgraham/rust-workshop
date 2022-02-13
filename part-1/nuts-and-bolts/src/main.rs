#![feature(stdin_forwarders)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

// Okay, now that we know the basics of "cargo", let's dive into the essential
// nuts and bolts that we enable us to build more complex programs.

// Again, we'll always start with our "main" function.
fn main() {
    // ******************** Data Types ********************

    // There are a handful of native types in Rust, with a level of granularity
    // that would be familiar to those using C++.

    // Signed integer types: i8, i16, i32, i64, i128, usize
    // Unsiged integer types: u8, u16, u32, u64, u128, isize
    // Float types: f32, f64
    // Logical: bool
    // Unicode character: char

    // Declaring variables is slightly different to how it is done in Python
    // and C++. In Rust we use the "let" keyword. Any type identification goes
    // after the variable name with a ":".
    let my_var: i32 = 100;

    // Though, in many cases the type can be inferred!
    let my_i32 = -100_000;
    // numeric literals where the type is vague can be followed by the type
    let my_u64 = 22u64;
    let my_f32 = 1.0f32;

    // But being explicit never hurts!
    let heart_kitty: char = '😻';

    // Though, I'm sure it will surprise everyone that these variables cannot be
    // modified! This fails (as it's clearly a contradiction)!
    // heart_kitty = '😿';

    // In order to "mutate" variable, we need to supply the "mut" keyword.
    let mut my_bool = true;
    // And then we can easily change this to false.
    my_bool = false;

    // So Rust has quite a different approach to mutability than Python and C++.
    // Where in Python you can do "anything" with a variable, and in C++
    // you can opt into "immutibility" with the "const" keyword, Rust asks that
    // you be aware of when a variable should be allowed to change after its
    // instantiation.

    // ******************** Compound Types ********************

    // We also have a few mechanisms for grouping multiple objects together into a
    // single variable. One of these is the tuple type.

    let my_tuple: (f32, u64, bool) = (3.141592, 6, true);

    // Another compound type is the array.

    let my_float_array: [f32; 3] = [1.0, 2.0, 3.0];
    let my_char_array: [char; 4] = ['a', 'b', 'c', 'd'];

    // ******************** Functions ********************

    // Now many times we want to encapsulate our code into neat blocks, i.e.
    // functions.

    // All functions start with the "fn" keyword, followed by their name and
    // curly braces to enclose the implementation
    fn hello_world() {
        println!("Hello, world!");
    }

    hello_world();

    // We often want these function to accept and return arguments.

    // Any arguments are defined by comma-separated list of "{name}: {type}",
    // and the return type follows with a "-> {type}".
    fn square(num: f32) -> f32 {
        num * num
    }

    let my_num = 2.0f32;
    let num_squared = square(my_num);

    println!("{my_num} squared is {num_squared}.");

    // ******************** Comments ********************

    // If it hasn't already been painfully obvious ... single-line comments are
    // precedded by a double-slash "//"

    // Hey look, I'm a single-line comment! I'm so (not) special.

    // Or can even define multiline comments with "/* ... */"

    /*
        I'm
        a
        multi-line
        comment
    */

    // ******************** Control Flow ********************

    // Control flow in Rust is pretty straight forward if you've programmed in
    // any other language

    let today = 19u8;

    if today == 20 {
        println!("🎂");
    } else if today == 19 {
        println!("🦀💻")
    } else {
        println!("🐱‍👓📚");
    }

    // We also have "match" statements that are great for when there are a finite
    // number of possible conditions! These will be really imporant when we cover
    // "enums"!

    let there_is_a_meteor_hurtling_towards_earth = true;

    match there_is_a_meteor_hurtling_towards_earth {
        true => println!("Everything is fine."),
        false => println!("Everything is fine."),
    }

    // And there are multiple ways that we can loop over data

    let mut count = 10;
    loop {
        if count == 0 {
            println!("Liftoff! 🚀");
            break;
        } else {
            println!("{count} ...")
        }

        count -= 1;
    }

    while count < 9 {
        count += 1;
        let tmp = (count as f32).sqrt();
        print!("{tmp} ")
    }
    println!();

    // "for" loops can be quite interesting, as they can accept a variety of
    // "iterator" types.

    // Range-based iteration
    for idx in 0..10 {
        print!("{idx} ")
    }
    println!();

    // Iteration over array elements
    for item in ['a', 'b', 'c', 'd'] {
        print!("{item}")
    }
    println!();

    // Or more abstract iterators
    println!("Type 'quit' to exit.");
    for line in std::io::stdin().lines() {
        if line.unwrap() == "quit".to_string() {
            break;
        }
        println!("Type 'quit' to exit.");
    }
}

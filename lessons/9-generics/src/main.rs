// Generic data types
//
// Generics allow us to create definitions for items like function signatures or structs, which can then be usd with different data types.
// In fact, we've implicitly used generics before: the vector itself is defined as a generic trait.

// Let's assume we have two functions that operate in a similar manner for different data types

fn main() {

    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}



// Since the functions both return the largest of their specific type, this makes an easy use case for a generic trait. 
// We define teh generic as something short; typically, devs use `T` as the default choice. 

// mod compile_error{
//     fn largest<T>(list: &[T]) -> T{
//         let mut largest = list[0];

//         for &item in list {
//             if item > largest {
//                 largest = item;
//             }
//         }
//         largest
//     }
// }

// When we compile we get an error, but a useful one
//    Compiling traits v0.1.0 (/home/chris/code/rust-workshop/part-1/traits)
// error[E0369]: binary operation `>` cannot be applied to type `T`
//   --> part-1/traits/src/main.rs:54:17
//    |
// 54 |         if item > largest {
//    |            ---- ^ ------- T
//    |            |
//    |            T
//    |
// help: consider restricting type parameter `T`
//    |
// 50 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T{
//    |             ++++++++++++++++++++++
//
// For more information about this error, try `rustc --explain E0369`.
// error: could not compile `traits` due to previous error

// It tells that the generic needs to implement a PartialOrd from the cmp (compare) portion of the standard library. This is how we tell the compiler that we want to the generic to implement a certain trait


mod no_error{
    fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T{
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    fn run_example() {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {result}.");

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&char_list);
        println!("The largest char is {result}.");
    }

}


// We can also use generics within the definitions of structs



// Note that as written, the above struct requires both entries to be of the same type. You cannot mix a float with an integer

mod partially_failing_example{
    struct Point<T> {
        x: T,
        y: T
    }
    fn point_error(){
        let integer = Point{x: 5, y:10};
        let float   = Point{x: 5.2, y:3.14159};
        // This one below fails
        // let mix     = Point{x: 1, y:1.41421546657};
    }
}

// This can be avoided if you two different different generics in the struct definitions 

mod no_fail {
    struct Point<T,U> {
        x: T,
        y: U
    }
    fn point_error(){
        let integer = Point{x: 5, y:10};
        let float   = Point{x: 5.2, y:3.14159};
        // This one below fails
        let mix     = Point{x: 1, y:1.41421546657};
    }
}


// We can also use them in Enum definitions. In fact, the `Option` enum implements it. 
// Let's check out the definition of Option from the standard library

mod options{
    // Copied from the standard library. 
    enum Option<T> {
        Some(T), 
        None,
    }
    
    // Enums can also have multiple generic types over its fields. A simple example is the Result class, which allows you to determine if a function encountered a problem or whether it ran successfully. 
    // Results are useful when something could fail and you want to handle errors. For example, when trying to open a file, you might not have read permissions or the file might not exist. Both would be errors you would want to handle instead of `panick`ing and crashing the program.

    enum Result<T, E> {
        Ok(T), 
        Err(E),
    }

    // Generics cna also be used in Method definitions
}

mod method {
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    fn test(){
        let p = Point {x: 5, y: 10};

        println!("p.x = {}", p.x());
    }

    // You can also provide additional functional for some constraint on the generic type.
    // Here, we declare an additional function for a Point<f32>

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

}

// YOu can also implement the generics when 

mod mixing_generics{
        struct Point<X1, Y1> {
        x: X1,
        y: Y1,
    }

    impl<X1, Y1> Point<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    fn generics() {
        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
}
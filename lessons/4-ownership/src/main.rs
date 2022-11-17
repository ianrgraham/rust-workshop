#![allow(unused_variables)]
#![allow(unused_assignments)]

// Now that we have a grasp of Rust syntax, let's get into a bit of the meat of
// the language. One of the critical concepts to grasp in Rust is that of
// ownership.

fn main() {
    // ******************** The "str" Type ********************

    // To explain ownership, we need to start handling more complex data types.
    // The first thing we'll talk about is the simple string literal. Prior to this
    // point, you'll notice we haven't used strings outside of the occasional
    // "println!" statement. When we assign a string to a variable, we find a new
    // sort of type that crops up, a "&str".

    let my_string: &str = "Hello, world!";

    // A "str" is the most primitve string type that Rust offers us, which ends
    // up being a simple sequence of unicode characters. But what does the "&"
    // mean? If you have any experience with C++, you'd recognize "&" as the
    // operator for defining references! That's essentially what we have here!
    // But to be more precise, "&str" is a fat pointer to some str (that may
    // live on the stack or heap) so it contains information about where the str
    // lives as well as its length.

    // We can print out our &str like any other type!
    println!("{my_string}");

    // Next lets cover the concept of scope. We can use a set of curly braces to
    // define a new scope. Any variables we define within the scope are not valid
    // outside of the scope.

    // We haven't yet seen "new_string", so cargo will complain
    // println!("{new_string}")
    {
        let new_string = "New!"; // "new_string" is now in scope & valid

        // Do stuff with the string.
        println!("{new_string}");
    } // "new string" is back out of scope, you can't do anything with it
      // println!("{new_string}")

    // So when variables are initialized, they come into scope and are "valid".
    // And variables remain valid until they go out of scope.

    // ******************** Extendable Strings ********************

    // Now, there isn't a lot you can do with a "&str" alone. We can't extend or
    // modify the str in any way. To achieve this, we need to apply a new type,
    // the "String".

    let mut my_string = String::from("Hello");
    println!("{my_string}");

    my_string.push_str(", world!");
    println!("{my_string}");

    // ******************** To Move or Not to Move ********************

    // It will be no surpise to you that the following code compiles just fine.
    let x = 5;
    let y = x;
    println!("{x}, {y}");

    // But surprisingly this doesn't!
    let a = "Hi!".to_string();
    let b = a;
    // println!("{a} {b}");

    // This is because "i32" and "String" are not quite treated the same way
    // when we assign one variable to another. In the case of i32 (and all other
    // primitive data types) it is "trivially copyable". So the behaviour of
    // assignment is to copy the data of "x" to "y". But when it comes to more
    // complex types, like String, copies can be quite a complex (and costly)
    // operation. Thus the default behaviour of these types is to NOT copy, but
    // to "move" the data to the new variable. We'll explain with an example
    // below.

    let s = "Wowie!".to_string(); // "s" owns the data of String("Wowie!")
    let s2 = s; // "s2" now takes ownership of the String, and s is now invalid
    println!("{s2}"); // We can continue to use String("Wowie!") using "s2"
    // println!("{s}"); // "s" is no longer valid! This doesn't work!

    // If we want to force a copy of the data, we can use the "clone" method.
    let a = "Hi!".to_string();
    let b = a.clone();
    println!("{a} {b}");

    // We can apply this thinking to functions. Here, the function takes ownership
    // of the argument String, which is then dropped at the end of the function

    fn take_ownership(s: String) {
        println!("I own {s:?}.");
    } // "s" goes out of scope and is thus dropped

    take_ownership(a);
    take_ownership(b);
    // println!("{a} {b}"); // a and b have been moved, so they are no longer valid

    // ******************** References and Borrowing ********************

    // Okay, but what if we don't want to move or copy the data? What if we want our
    // new variable to reference our old data? Well, we can do just that!

    let a = "Hi!".to_string();
    let b = &a; // "b" is now of type "&String", not "String"
    println!("{a} {b}");

    // This operation is a "borrow". So "b" now holds an immutable reference to
    // "a". Like with the move, we can perform borrows with the call signature
    // of a function

    fn borrow_string(s: &String) {
        println!("I only borrow {s:?}.")
    }

    borrow_string(&a);
    borrow_string(b);
    // The variables are still valid after a borrow
    println!("{a} {b}");

    // What about if we want to take a mutable reference? We just use "&mut".

    let mut s = "Mutate me!".to_string();

    let s2 = &mut s;
    s2.push_str(" Mutated!");

    println!("{s}"); // Can print the original variable, now changed through a ref

    // Though there are restrictions here ... When we take a mutable borrow, we
    // are also promising to the compiler that no other variable is borrowing
    // from the original data simultaneously.

    let mut s = String::new();

    let s2 = &mut s;

    // let s3 = &s; // This is not allowed

    s2.push_str("Hey!");

    // The reason for this restriction may not be clear at this moment, but there
    // are cases where mutation can invalidate a prior reference, e.g. reallocation.
    // We're going show this off using a new data type, the "Vec"!

    let mut v = vec![1]; // Allocates a vector of size 1

    // It's illegal to do what we are doing below, and for a good reason! 
    let v2 = &v[0] as *const i32; // Get raw ptr to first index
    let v3 = unsafe { v2.as_ref().unwrap() }; // Transform into a ref
    // "v3" is still valid since we haven't modified "v"
    println!("Before reallocation, {} == {}", v[0], v3);

    // But if we suddenly ask "v" to reserve a lot of space, this should force
    // the data structure to move somewhere else in memory
    v.reserve(100_000); // This should force a reallocation

    // The data no longer matches!
    println!("After reallocation, {} == {}", v[0], v3);
}

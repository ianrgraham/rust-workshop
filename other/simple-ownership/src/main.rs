fn main() {
    // The Rust language formalizes a concept that programmers often encounter
    // (knowningly or not): Ownership.
    // Every time we have shared data in a low level language like C we have
    // some idea of what bits of the code are responsible for it's management:
    // allocation, initialization, dealloction. We understand that most times
    // a well written block of code will not leak memory outside of it's scope,
    // or even worse reference memory that is no longer valid. Though the
    // programmers mental model isn't bullet proof, and sometimes we have a lapse
    // in reasoning about our code (or we just forget to read the documenation correctly).
    // Rust doesn't stand for a mental model rule of thumb. Instead, the compiler
    // enforces specific requirements on how data is handled throughout a program
    // which in turn greatly reduce the chance of some very common memory bugs
    // cropping up, and eases the burden on the poor programmer.

    // When we initialize some data using `let`, the variable (or rather it's scope)
    // clearly holds ownership of it. my_string is the `owner` of String("Hello!").
    let mut my_string = String::from("Hello!");

    // Having data accessable by only one variable isn't too useful. If we want 
    // other parts of our code to access it, we can _lend out_ the data for a short
    // time. If all we would like to do is read the data (no writing!) we can 
    // perform an "immutable borrow".
    let my_reference = &my_string;

    // we can have as many immutable borrows as we'd like
    let another_reference = &my_string;

    // With an immutable borrow we can read from both variables simultaneously
    println!("{} {} {}", my_string, my_reference, another_reference);

    // We can also pass a mutable reference of my_string to another variable. This operation 
    // is a "mutable borrow". While it may not appear that this new variable can modify our
    // original data, it infact can (`borrows_my_string` is a const pointer to 
    // _mutable_ data!)
    let borrows_my_string = &mut my_string;

    // while not obvious, Rust has auto-dereferring builtin to many types
    // no need to us `->`! The meaning is almost always clear that you inted on
    // applying the function to T and not &T
    borrows_my_string.push_str(" World!");
    println!("{}", borrows_my_string);

    // This doesn't work! `borrows_my_string` is a const pointer! Must dereference
    // if you actually want to modify the borrowed value
    // borrows_my_string = String::from("Why?")

    // Neither can we use `my_string` while the borrow is occurring. The borrow
    // is essentially a temporary contract between the variables that ensures
    // println!("{}", my_string);

    // This does work! Replaces the borrowed value with a new string.
    *borrows_my_string = String::from("Why?");

    // Implicit drop of `borrows_my_string` allows us to use `my_string` again!
    // std::mem::drop(borrows_my_string)

    println!("{}", my_string);
}

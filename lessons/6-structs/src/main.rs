// In Rust, structs are something of a middle-ground between the Object Oriented
// world of C++ and Python and data structs from C. Think of them like "structs
// with methods". The one feature they don't have is inheritance, but Rust has
// it's own facilities for programming shared behaviour (that we'll get to
// later)!

// Simple complex data type
#[derive(Debug)]
struct Complex64 {
    re: f32,
    im: f32,
}

// This is where any methods relating to a struct go
impl Complex64 {
    // Most stucts will have "new" or "from_<type>" or "with_<requirement>"
    // methods to build the struct. This example is simple, but you could imagine
    // a lot more code entering in here before `Self` is returned
    fn new(re: f32, im: f32) -> Self {
        Complex64 { re, im }
    }

    // We can also define methods that take `&self` as their first argument.
    // These methods are called from instantiations of the struct.
    fn norm(&self) -> f32 {
        (self.re.powi(2) + self.im.powi(2)).sqrt()
    }

    // Also, our methods can mutate the state of our structs
    fn rotate(&mut self, ang: f32) {
        let sin = ang.sin();
        let cos = ang.cos();
        self.re = cos * self.re - sin * self.im;
        self.im = sin * self.re + cos * self.im;
    }

    // Or our methods can consume the struct in the process
    fn nuke_via_haiku(self) {
        print!("life of a struct\nmethods respect borrow checker\nbye bye bye")
    }
}

fn main() {
    let mut x = Complex64::new(1.0, 0.0);

    println!("The norm of {:?} is {}", x, x.norm());

    println!("{}", -1.0f32.sin());

    // This printing isn't so nice, but how oh how can we make our custom function
    // print like u32 or &str? Is there some class we can inherit? Not a class
    // Timmy! But a trait! Rust's way of building shared behaviour is through
    // the machinery of traits! These are similar to interfaces if you've ever used
    // Java. Essentially, traits define an interface that a type must have to
    // implement said trait, but the implementation is up to the creator of the
    // struct! This is in stark contrast to how inheritance is used to obtain
    // shared behaviour in C++ and Python, as implementations of functions from
    // the base class waterfall onto the child class. Sometimes, this is exactly
    // what the programmer wants! But in many other cases it is known to cause
    // problems.

    x.rotate(3.141592*6.0/4.0);

    println!("The norm of {} is {}", x, x.norm());

    x.nuke_via_haiku()
}

// This is an example implementation of the Display trait for our Complex64 struct.
impl std::fmt::Display for Complex64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let sign = if self.im >= 0.0 { "+" } else { "-" };

        write! {f, "{:.1}{}{:.1}i", self.re, sign, self.im.abs()}?;
        Ok(())
    }
}

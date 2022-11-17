// Here we cover traits. Traits guarantees for the compiler that the object (which can be any `Self`) implements specific features. We've previously seen the fmt::Display trait for the Complex64.

// The specific implementation of the guaranteed behavior is type specific. Displaying a string is different from displaying our Complex64.

// Let's construct a struct called `Sheep` and define the `Animal` trait.
struct Sheep {
    naked: bool,
    name: &'static str,
}
struct Dog {
    naked: bool,
    name: &'static str,
}

trait Animal {
    // Associated function signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    // Method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

// Implementing `Sheep` and its corresponding functions
impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // Implementor methods can use the implementor's trait methods.
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    // `Self` is the implementor type: `Sheep`.
    fn new(name: &'static str) -> Sheep {
        Sheep {
            name: name,
            naked: false,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }

    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

// Implement the `Animal` trait for `Dog`.
impl Animal for Dog {
    // `Self` is the implementor type: `Sheep`.
    fn new(name: &'static str) -> Dog {
        Dog {
            name,
            naked: false,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        "Woof"
    }

    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    // Type annotation is necessary in this case.
    let mut dolly: Sheep = Animal::new("Dolly");
    // TODO ^ Try removing the type annotations.

    dolly.talk();
    dolly.shear();
    dolly.talk();

    let fido = Dog::new("Fido");

    fido.talk();
}


// We can also define our traits for external types (types we didn't define, such as the Vec<T> or Option<T>). We can use external traits like Display
// For our own traits, we can define default implementations within the declaration block

pub trait Summary{
    fn summarize(&self) -> String{
        String::from("Here's a summary snippet!")
    }
}

impl Summary for Sheep {
    fn summarize(&self) -> String {
        String::from("I am a sheep and my name is {self.name}")
    }
}

impl <T> Summary for Vec<T>{
    fn summarize(&self) -> String {
        String::from("I have {self.len()} elements")
    }
}

use std::fmt::Display;

// You can also use Traits as a parameter for a function
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// This can alternatively written as 
pub fn notify_v2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// You can string them together
pub fn notify_v3<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Or explicitly write teh trait types
pub fn notify_v4<T>(item: &T) 
where T: Summary + Display {
    println!("Breaking news! {}", item.summarize());
}


// You can also use the traits as a condition for the output
pub fn notify_v5() -> impl Summary{
    Sheep {
        name: "Jim",
        naked: false,
    }
}

// You can also conditionally implement traits based on the type


struct Pair<T> {
    x: T,
    y: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
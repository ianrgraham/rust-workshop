use std::fmt;

#[derive(Debug)]
enum Message {
    // Exercise! implement the Move, Echo, ChangeColor, and Quit variants
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Exercise! implement the pretty_call method
            // Note: use the write! macro instead of println!
        }
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    println!("Debug print:");
    for message in &messages {
        message.call();
    }

    println!("\nPretty print:");
    for message in &messages {
        println!("{}", message);
    }
}

use std::fmt;

#[derive(Debug)]
enum Message {
    Move { x: i32, y: i32 },
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
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
            Message::Move { x, y } => write!(f, "Move to ({}, {})", x, y),
            Message::Echo(s) => write!(f, "Echo: {}", s),
            Message::ChangeColor(r, g, b) => write!(f, "Change color to ({}, {}, {})", r, g, b),
            Message::Quit => write!(f, "Quit"),
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

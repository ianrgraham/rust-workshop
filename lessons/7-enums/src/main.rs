// In Rust, enums (enumerations) are similar to enums from C and Python, but are
// more robust. You can mix and match the values stored within enums, instead of
// each value of enum being an abstracted integer value. Additionally, the
// compiler checks each possible value of an enum for things like match
// statements, ensuring that you cover all cases. addressed. Enums also allow
// you to mix and match the types stored within an enum, which is different

enum Colors {
    Red,
    Blue,
    Green,
    Other(String),
}

fn main() {
    // This is useful for the Rust equivalent of "switch" statements, the match
    // statement. Match statements require that all cases be covered.

    let fav_color: Colors = Colors::Red;
    match &fav_color {
        Colors::Red => println!("My favorite color is red!"),
        Colors::Blue => println!("My favorite color is blue!"),
        Colors::Green => println!("My favorite color is green!"),
        Colors::Other(color) => {
            println!("My favorite color is not red, blue or green! My favorite color is {color}!")
        }
    };
    println!("");

    // let fav_color = Colors::Red;
    match &fav_color {
        Colors::Red => println!("My favorite color is red!"),
        // Colors::Blue => println!("My favorite color is blue!"),
        Colors::Other(color) => {
            println!("My favorite color is not red, blue or green! My favorite color is {color}!")
        }
        _ => println!("My favorite color is blue or green! I don't remember which."),
    };
    println!("\n");



    {
        // The Option struct is a special struct used for when a variable might not hold
        // a variable. This might happen when the user hasn't provided an input, as a
        // return value for otherwise reporting simple errors, where None is returned on
        // error, or Nullable pointers, Optional struct fields
        //
        // Has two possible values: None or Some<T> where T is a generic

        fn divide(numerator: f64, denominator: f64) -> Option<f64> {
            if denominator == 0.0 {
                None
            } else {
                Some(numerator / denominator)
            }
        }

        // The return value of the function is an option
        let result: Option<f64> = divide(2.0, 3.0);

        // Pattern match to retrieve the value
        match result {
            // The division was valid
            Some(x) => println!("Result: {}", x),
            // The division was invalid
            None => println!("Cannot divide by 0"),
        }
    }

    // Let's use Options on our Color enums and explore the possible uses

    let option_fav_color1: Option<Colors> = Some(Colors::Blue);
    let option_fav_color2: Option<Colors> = Some(Colors::Red);
    let option_fav_color3: Option<Colors> = None;

    let fav_colors = vec![option_fav_color1, option_fav_color2, option_fav_color3];

    // Here we need to pass a reference to the vector so we don't become the owners
    // of the vector
    for fav in &fav_colors {
        match fav {
            Some(Colors::Red) => println!("My favorite color is red"),
            Some(_) => println!("I have a favorite color"),
            None => println!("I have no favorite color :'(  "),
        };
    }
    println!("");

    // Here's another variation of taking a reference to the vector
    // This time, we introduce some of the power of pattern matching. We are able to
    // pattern match on the values within teh
    for fav in fav_colors.iter() {
        match fav {
            Some(Colors::Red) => println!("My favorite color is red"),
            Some(Colors::Green) | Some(Colors::Blue) => {
                println!("My favorite color is blue or green! I don't remember which.")
            }
            Some(_) => println!("I have a favorite color"),
            None => println!("I have no favorite color :'(  "),
        };
    }
    println!("");

    // Since the vector is not usd after this for loop, we are able to iterate
    // directly on the vector without worrying about ownership. If we copied and
    // pasted this loop directly, the compiler would complain

    let ora_nge = "orange";

    let mut fav_colors = fav_colors;
    fav_colors.push(Some(Colors::Other("orange".to_string())));

    for fav in fav_colors {
        match fav {
            Some(Colors::Red | Colors::Green) => {
                println!("My favorite color is red or green.")
            }
            // This match below is unreachable
            Some(Colors::Green) => println!("My favorite color is blue!"),
            Some(Colors::Other(oth_color)) => match oth_color.as_str() {
                "yellow" => println!("My favorite color is yellow!"),
                ora_nge => println!("My favorite color is orange!"),
                _ => println!("My favorite color is something."),
            },
            Some(_) => println!("I have a favorite color"),
            None => println!("I have no favorite color :'(  "),
        };
    }
    println!("");

    // If the match would have a single arm, you can use an `if let` statement!
    // For the `if let`, the matchign statement comes first and the variable you are testing against comes second, which is the opposite order of the match.
    // Here, you are testng if you can assign the left hand with right hand value. 
    // Also, like other `if` statements, you can chain an `else` to the end. 

    let color = Colors::Red;

    if let Colors::Other(string) = color {
        println!("{string} is my color");
    } else {
        println!("I like RGB.")
    }

}

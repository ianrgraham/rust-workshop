
#[derive(Debug)]
struct MyStruct<'lifetime_of_ext_ref> {
    a: &'lifetime_of_ext_ref str
}


fn main() {
    // string x = string("Hello");

    let my_str = String::from("Hi");

    let tmp = MyStruct {a: &my_str};

    std::mem::drop(my_str);

    println!("{tmp:?}");
    println!("{:?}",tmp);

    // println!("{my_str}");


    // std::string str1("Example");

    let mut x: String = String::from("Hello!");
    
    let mut z: String = String::from("yomama.txt");
    println!("x = {x}");

    let mut y: &mut String = &mut x; // explicit borrow of x now. y owns x
    y.push_str("blah");
    y =  &mut z;
    i_borrow(&y);
    // implicit drop of y now
    println!("x = {x}");
    // x = String::from("hi!");

    {
        let y: String = String::from("string");
        println!("y = {y}");
        // println!("x = {x}");
    }

    // i_borrow(&y);
    i_borrow(&y);
    println!("x = {x}");
}

fn i_borrow(item: &String) {
    println!("{item}");
}

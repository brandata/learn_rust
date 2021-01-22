use std::io::{self};

fn main() {
    let mut input_string = String::new();

    println!("Enter some text:");

    match io::stdin().read_line(&mut input_string) {
        Ok(_) => println!("You typed {}", input_string),
        Err(error) => println!("Error: {}", error),
    };
}

mod calculator;
mod temprature;

use std::io;

fn main() { // The Selector
    let mut input: String = String::new(); // Creates a mutable input string

    println!("Silly multitool thingy \n 1. Calculator \n 2. Temprature Convertion \n 3. Color Selector \n 4. GIF recorder \n 5. Video to GIF");

    io::stdin().read_line(&mut input).expect("Failed to read line"); // Tries to read user input

    let input: u8 = input.trim().parse().expect("Please input a number."); // Shadows the input string, tries to convert the string to an unsigned 8-bit integer and then stores it

    match input { // Switch statement in Rust
        1 => { calculator::calculation(); main(); }
        2 => { temprature::convertion(); main(); }
        3 => println!("Color selector"),
        4 => println!("GIF recorder"),
        5 => println!("Video to GIF"),
        _ => println!("Please enter one of the numbers listed."),
    }
}
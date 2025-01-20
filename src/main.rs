use std::io;

fn main() {
    let mut input = String::new(); // Creates a mutable input string

    println!("Silly multitool thingy \n 1. Calculator \n 2. Color Selector \n 3. GIF recorder \n 4. Video to GIF");

    io::stdin().read_line(&mut input).expect("Failed to read string"); // Tries to read user input

    let input: u8 = input.trim().parse().expect("Please input a number."); // Shadows the input string, tries to convert the string to an unsigned 8-bit integer and then stores it

    match input { // Switch statement in Rust
        1 => println!("Calculator"),
        2 => println!("Color Selector"),
        3 => println!("GIF recorder"),
        4 => println!("Video to GIF"),
        _ => println!("Please enter one of the numbers listed."),
    }
}


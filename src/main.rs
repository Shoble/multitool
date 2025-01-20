use std::io;

fn main() {
    let mut input: String = String::new(); // Creates a mutable input string

    println!("Silly multitool thingy \n 1. Calculator \n 2. Color Selector \n 3. GIF recorder \n 4. Video to GIF");

    io::stdin().read_line(&mut input).expect("Failed to read line"); // Tries to read user input

    let input: u8 = input.trim().parse().expect("Please input a number."); // Shadows the input string, tries to convert the string to an unsigned 8-bit integer and then stores it

    match input { // Switch statement in Rust
        1 => calculator(),
        2 => println!("Color Selector"),
        3 => println!("GIF recorder"),
        4 => println!("Video to GIF"),
        _ => println!("Please enter one of the numbers listed."),
    }
}

fn calculator() {
    println!("Enter the first part of the calculation, then the operator and then the last part of the calculation. 
You can reuse the result of the previous calculation by leaving the input blank. You can return to the selector at any point by entering back.");
    let mut result: f64 = 0.0;
    loop {
        let mut first:String = String::new();
        let mut operator:String = String::new();
        let mut last:String = String::new();

        io::stdin().read_line(&mut first).expect("Failed to read line.");
        if first.trim() == "back" { break; }
        let first: f64 = match first.trim().parse() {
            Ok(num) => num,
            Err(_) => result,
        };
    
        io::stdin().read_line(&mut operator).expect("Failed to read line");
        if operator.trim() == "back" { break; }

        io::stdin().read_line(&mut last).expect("Failed to read line.");
        if last.trim() == "back" { break; }
        let last: f64 = match last.trim().parse() {
            Ok(num) => num,
            Err(_) => result,
        };

        match operator.trim() {
            "+" => result = first + last,
            "*" => result = first * last,
            "/" => result = first / last,
            "-" => result = first - last,
            "%" => result = first % last,
            _ => println!("Please input a valid operator"),
        }
        println!("= {}", result);
    }
    main();
}
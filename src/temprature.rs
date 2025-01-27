use std::io;

pub fn convertion() {
    loop { // Loop so you can convert multiple times
        // Creating user input string
        let mut input_temp: String = String::new();

        println!("Temprature convertion between Celsius and Fahrenheit. Please enter the temprature you would like to convert or enter back to return to the selector:");

        io::stdin().read_line(&mut input_temp).expect("Failed to read line."); // Taking user input
        if input_temp.trim() == "back" { break; } // Checking if user wants to return to selector
        let input_temp: f64 = input_temp.trim().parse().expect("Please enter a number"); // Converts user input to number

        println!("{input_temp}°F in Celsius is {}°C", convert_fahrenheit(input_temp));
        println!("{input_temp}°C in Fahrenheit is {}°F", convert_celsius(input_temp));
    }
}

fn convert_fahrenheit(temp: f64) -> f64 { // Converts Fahrenheit to Celsius
    let convertion: f64 = 5.0 / 9.0;
    let temp: f64 = temp - 32.0;
    temp * convertion
}

fn convert_celsius(temp: f64) -> f64 { // Converts Celsius to Fahrenheit
    temp * 1.8 + 32.0
}
use std::io;

fn main() {
    const CELSIUS_TO_FAHRENHEIT: u32 = 1;
    const FAHRENHEIT_TO_CELSIUS: u32 = 2;

    let temperature: f64 = loop {
        println!("Please input a temperature.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Reading temperature failed!");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };

    println!("Which conversion would you like?");
    println!("  1. Celsius to Fahrenheit");
    println!("  2. Fahrenheit to Celsius");

    let choice: u32 = loop {
        println!("Input your choice: ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Reading choice failed!");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if input == CELSIUS_TO_FAHRENHEIT || input == FAHRENHEIT_TO_CELSIUS {
            break input;
        }
    };

    println!("Your temperature: {temperature}, your choice: {choice}");

    let temperature = if choice == CELSIUS_TO_FAHRENHEIT {
        temperature * 9.0 / 5.0 + 32.0
    } else {
        (temperature - 32.0) * 5.0 / 9.0
    };

    println!("Temperature after conversion: {temperature}");
}

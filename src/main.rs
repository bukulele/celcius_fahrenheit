// celcius to fahrenheit -> Fahrenheit = (Celsius * 1.8) + 32
// fahrenheit to celcius -> Celsius = (Fahrenheit - 32) / 1.8

use std::io;

fn main() {
    const MODES: [&str; 2] = ["Celcius", "Fahrenheit"];
    let mut mode: usize = 0;

    println!("Hi! This is Celcius-Fahrenheit converter.");

    println!("If you want to change mode, input 'chm' and press Enter");
    println!("If you want to exit, input 'quit' and press Enter");

    let mut user_input = String::new();

    loop {
        user_input.clear();

        if mode == 0 {
            println!("Current mode: C -> F");
        } else {
            println!("Current mode: F -> C");
        }

        println!("Input temperature to convert and press Enter");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        if user_input.trim() == "chm" {
            if mode == 0 {
                mode = 1;
            } else {
                mode = 0;
            }
            continue;
        }

        if user_input.trim() == "quit" {
            break;
        }

        let user_input: f64 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number or a correct command");
                continue;
            }
        };

        if mode == 0 {
            let temperature_calculated = user_input * 1.8 + 32.0;
            println!(
                "{} {} is equal to {} {}",
                user_input, MODES[mode], temperature_calculated, MODES[1]
            );
        } else {
            let temperature_calculated = (user_input - 32.0) / 1.8;
            println!(
                "{} {} is equal to {} {}",
                user_input, MODES[mode], temperature_calculated, MODES[0]
            );
        }

        continue;
    }
}

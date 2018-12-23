use std::io;

fn c_to_f(celsius: i32) -> i32 {
    celsius * 9 / 5 + 32
}

fn f_to_c(fahrenheit: i32) -> i32 {
    (fahrenheit - 32) * 5 / 9
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input!");
    input
}

fn request_temp_input() -> i32 {
    let mut is_valid_temperature: bool = false;
    let mut result: i32 = 0;
    println!("Please enter a temperature");
    while !is_valid_temperature {
        let input = read_input();
        match input.trim().parse() {
            Ok(num) => {
                is_valid_temperature = true;
                result = num;
            }
            Err(_) => {
                println!("Invalid input, please try again!");
                continue;
            }
        }
    }
    result
}

fn request_input_type() -> bool {
    loop {
        println!("What do you want to calculate?");
        println!("1: Fahrenheit => Celsius");
        println!("2: Celsius => Fahrenheit");
        let input = read_input();
        if input.trim() == "1" {
            break true;
        } else if input.trim() == "2" {
            break false;
        }
    }
}

fn main() {
    println!("Welcome to the Fahrenheit <-> Celsius converter!");
    println!();

    loop {
        let input_is_f = request_input_type();
        let temperature_input = request_temp_input();
        let input_unit_name = if input_is_f { "Fahrenheit" } else { "Celsius" };
        let output_unit_name = if input_is_f { "Celsius" } else { "Fahrenheit" };

        let temperature_result = if input_is_f {
            f_to_c(temperature_input)
        } else {
            c_to_f(temperature_input)
        };

        println!(
            "{}°{} = {}°{}",
            temperature_input, input_unit_name, temperature_result, output_unit_name,
        );
        println!();
    }
}

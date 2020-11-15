

use simple_user_input::get_input;

/// Calculated using formulae from https://people.maths.ox.ac.uk/trefethen/bmi.html
fn calculate_bmi(height: f32, weight: f32) -> f32 {
    // BMI = 1.3*weight(kg)/height(m)^2.5 = 5734*weight(lb)/height(in)^2.5
    (1.3 * weight) / height.powf(2.5)
}

fn calculate_status(bmi: f32) -> String {
    if      bmi < 18.5  {"too thin".into()}
    else if bmi < 25.0  {"healthy".into()}
    else                {"overweight".into()}
}


mod simple_user_input {
    use std::io::{stdin, stdout, Write};
    pub fn get_input(prompt: &str) -> String{
        print!("{} ",prompt);
        stdout().flush().expect("Flush stdout");
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }
}

fn main() {
    println!("Welcome to BMI Calculator\n");

    let height = get_input("Height (m)?").parse().expect("Couldn't parse height.");
    let weight = get_input("Weight (kg)?").parse().expect("Couldn't parse weight.");

    let bmi = calculate_bmi(height, weight);
    println!("\nYour BMI is {}", bmi);
    println!("You are {}\n", calculate_status(bmi));
}
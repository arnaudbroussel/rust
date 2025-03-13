use std::io::{self};
use rustworkshop::*;

fn main() {
    let mut number_str = String::new();
    text_to_console("Please enter your number: ");

    // Read input correctly
    io::stdin().read_line(&mut number_str).unwrap();

    // Parse the input safely
    let number: i32 = match number_str.trim().parse() {
        Ok(num32) => num32,
        Err(_) => {
            end_program("Invalid input!\nPlease enter a valid integer.\nPress Enter to exit...");
            return;
        }
    };

    let message = format!("Calculation: {}.", add_constant(number));
    text_ln_to_console(&message);
    text_ln_to_console(&format!("Calculation (output of the expression, not a variable): {}.", add_constant(number)));

    end_program("Program finished. Press Enter to exit...");
}
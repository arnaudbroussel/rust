mod utils;
use utils::console_io::*;
use utils::primitive_types::*;
use utils::compare_with_csharp::*;
use utils::serialize_deserialize::*;
use rustworkshop::*;

fn main() {
    menu();
}

fn menu(){
    text_ln_to_console("*** MENU ***\n");
    text_ln_to_console("(1) Serialize Deserialize");
    text_ln_to_console("(2) Primitive types");
    text_ln_to_console("(3) Fibonacci");
    text_ln_to_console("(4) Sum of Large Array (Memory & Loop Performance)");
    text_ln_to_console("(5) Multithreading Test)");
    text_ln_to_console("(...) n/a");
    text_ln_to_console("(9) Original code");
    text_ln_to_console("(0) Exit");
    
    text_to_console("\nPlease enter your choice: ");

    let mut menu_choice = String::new();
    input_from_console(&mut menu_choice);
    
    match menu_choice.trim() {
        "1" => serialize_deserialize_examples(),
        "2" => work_with_primitive_types(),
        "3" => run_fibonacci(),
        "4" => sum_of_large_array(),
        "5" => multi_threading(),
        "9" => original_code(),
        _ => end_program("\nPress Enter to exit..."),
    }

    pause_program("\nPlease press Enter to come back to menu.\n");
    
    menu();
}

fn original_code(){
    serialize_deserialize_examples();

    text_to_console("Please enter your number: ");

    let mut number_str = String::new();
    input_from_console(&mut number_str);

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
}
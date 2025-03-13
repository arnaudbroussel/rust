use std::io;
use std::io::Write;

const MY_ROOT_NUMBER: i32 = 400;

pub fn add_constant(input: i32) -> i32 {
    input + MY_ROOT_NUMBER
}


pub fn text_ln_to_console(text: &str) {
    println!("{}", text);
    flush_text();
}
pub fn text_to_console(text: &str) {
    print!("{}", text);
    flush_text();
}

// Print the prompt and flush to ensure it appears before input
fn flush_text(){
    io::stdout().flush().unwrap();
    return;
}

pub fn end_program(text: &str) {
    text_to_console(text);

    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();
}
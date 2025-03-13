use std::io;
use std::io::Write;

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
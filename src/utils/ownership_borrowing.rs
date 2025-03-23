pub fn ownership_borrowing() {
    // Moves / Copies primitives / structs
    let a: i32 = 1;
    let b: i32 = a;
    // The primitive value type is implicitly copied
    println!("a: {}", a);
    println!("b: {}", b);
    
    let string_a: String = String::from("Hello");
    let string_b: String = string_a;
    // Uncommenting this code will cause the program to not compile
    println!("string_b {}", string_b);
    
    //// follow from here
}
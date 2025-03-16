pub fn work_with_primitive_types() {
    // Integers
    let my_num: i32 = 32;
    println!("Integer: {}", my_num);
    
    // Converting strings to integers
    let mut parsed_num: i32 = "1234".parse().unwrap();
    parsed_num = parsed_num+1;
    println!("Parsed num: {}", parsed_num);
    println!("Integer to string: {}", parsed_num.to_string());

    // Floats
    let my_float: f32 = 10.5;
    println!("Float: {}", my_float);
    println!("Float floor: {}", my_float.floor());
    println!("Float ceil: {}", my_float.ceil());
    println!("Float round: {}", my_float.round());
    
    // working with floats and integers
    let my_int: i32 = my_float as i32 + 1;  
    let my_new_float: f32 = 115f32 + my_float as f32; // 115 as f32
    println!("Coercing float to int: {}", my_int);
    println!("Coercing int to float: {}",  my_new_float);
    
    // Characters
    let my_char: char = 'A';
    println!("Char: {}", my_char);
    println!("Char is_uppercase: {}", my_char.is_uppercase());
    println!("Char is_lowercase: {}", my_char.is_lowercase());
    println!("To lowercase: {}", my_char.to_string());
    println!("To string: {}", my_char.to_string());
    
    // Booleans
    let my_bool: bool = true;
    assert_eq!(my_bool, true);
    
    // Tuples
    let my_tuple_1: (i32, char, f32) = (1, 'A', 10.5f32);
    let my_tuple_2= (1, 'A', 10.5f32);  // type not specified
    println!("Integer/Char/Float: {}/{}/{}", my_tuple_1.0, my_tuple_1.1, my_tuple_1.2);
    println!("Integer/Char/Float: {}/{}/{}", my_tuple_2.0, my_tuple_2.1, my_tuple_2.2);
    
    // Destructuring tuples values into variables
    let (integer_1, letter_1, float_num_1) = my_tuple_1;
    let (integer_2, letter_2, float_num_2) = my_tuple_2;
    
    let nested_tuple_1: ((i32, i32), (i32, i32)) = ((1,2),(3,4));
    let nested_tuple_2 = ((1,2),(3,4));

    let ((num1_1,num2_1),(num3_1,num4_1)) = nested_tuple_1;
    let ((num1_2,num2_2),(num3_2,num4_2)) = nested_tuple_2;
    
    // Arrays
    println!("\nArrays");
    let my_array: [i32; 4] = [num1_1, num2_1, num3_1, num4_1];
    for item_from_array_iter in my_array.iter() {
        println!("item_from_array_iter: {}", item_from_array_iter);
    }
    print!("\n");
    for number_from_array in my_array {
        println!("number_from_array: {}", number_from_array);
    }
    
    let mut same_value_array: [i32; 100] = [10; 100]; // fill an array of i32 with 1000 times the value 10
    println!("\nsame_value_array {:?}", same_value_array);
    println!("First element of same_value_array {}", same_value_array[0]);
    println!("Last element of same_value_array {}", same_value_array[same_value_array.len() - 1]);
    println!("Size of same_value_array {}", std::mem::size_of_val(&same_value_array));

    same_value_array[0] = 99;
    same_value_array[same_value_array.len() - 1] = 88;
    println!("\nsame_value_array {:?}", same_value_array);
}
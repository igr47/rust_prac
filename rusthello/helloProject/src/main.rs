// Primitive data types 
// int, float, bool, char 
//
// Integer 
// Rust has signed and unsigned integer 
// Signed means it can have (- or +) but unsigned means it is only positive
// i8, i16, i32, i64, i128 These are the signed integers. They can hold either positive or negative
// u8, u16, u32, u64, u128 These are unsigned meaning They are only positive.

fn main() {
    let x: i32 = -42;
    let y: u32  = 100;
    println!("Signed integer: {}", x);
    println!("Unsigned integer: {}", y);

    // Float only has 2 types
    // f32, f64
    let pi: f64 = 3.14;
    println!("Float data type: {}", pi);

    let is_snowing: bool = true;
    println!("Is snowing: {}", is_snowing);

    let letter: char = 'a';
    println!("First letter of the alphabet: {}", letter);

    let numbers: [i32; 5] = [0, 1, 2, 3, 4];
    println!("Numbers are: {:?}", numbers);

    //let mix: [i32; 4] = [1, 2, "apple", true];
    //println!("Mixture is: {}", mix);

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array: {}", fruits[0]);
    println!("Fruits Array: {}", fruits[1]);
    println!("Fruits Array: {}", fruits[2]);

    // Tuples can have a mixture of data types
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human tuple {:?}", human);

    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number slices: {:?}", number_slices);

    let book_slices: &[String] = &["IT".to_string(), "Harry Porter".to_string(), "ZEN".to_string()];
    println!("Book slices: {:?}", book_slices);

    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("yeah");
    println!("Stone cold says {}", stone_cold);

    let string: String = String::from("Hello, world");
    let slice: &str = &string[0..5];
    println!("String slice: {}", slice);
}

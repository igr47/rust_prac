
fn main() {
    hello_world();
    tell_height(32);
    human_id("Elvis", 20, 180.0);
    let x: i32 = add_numbers(4, 6);
    println!("Addition: {}", x);
}

fn hello_world() {
    println!("Hello, rust");
}

fn tell_height(height: u32) {
    println!("My height is: {}", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!("My name is {}, I am {} years old, and my height is {} cm", name, age, height);
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

// Expressions and statements 
// Expressions is anything that returns a value
// A statement is anything that does not return a value
// Expressions 
// 5
// true & false - boolean expressions 
// functions 
// if conditions 
// code blocks 


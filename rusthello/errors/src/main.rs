// The first method of handling errors in rust is using OPTION<T>
// OPTION<T> is basically an enum and is used when a value might not be present.
// This solves the issue of null references.
// To use it we create an enum with the keyword option after which will take Some(T), or None
// enum Option<T> { // Defines the generic  Option type 
//  Some(T),  // Represents a value 
//  None,     // Represents no value
// }
// The second approach is called Result which is an enum 
// it takes two generic values T and E as generic parameters.
// Inside the enum it has two constructors Ok(T) and Err(E)
// enum Result<T, E> { // Defines the generic result type 
//  Ok(T),     // Represents a value 
//  Err(E),    // Represents an error 
// }

use core::f64;

fn divide(numerator: f64, denomanator: f64) -> Option<f64> {
    if denomanator == 0.0 {
        None
    } else {
        Some(numerator / denomanator)
    }
}

fn divide_result(numerator: f64, denomanator: f64) -> Result<f64, String> {
    if denomanator == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(numerator / denomanator)
    }
}

fn main() {

    let result: Option<f64> = divide(10.0, 2.0);
    match result{
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by zero"),
    }

    let result_2 = divide_result(100.0, 20.0);
    match result_2 {
        Ok(x) => println!("Result: {}", x),
        Err(e) => println!("Error: {}", e),
    }
}

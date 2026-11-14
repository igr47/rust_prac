// Lifetimes are used to show the relationship between lifetimes. 
// To declare Generic Lifetime Annotations we add a <> after the function name
// and in the brackets name the lifetime  wich is precedented with an ' 
// The name can be anything 
// fn largest<'a>(x: &'a str, y: &'a str) -> &a' str 
// Mostly the compiler will be able to deduce the lifetimes in the case off one 
// parameter. The issue stems from 2 parameters since the compiler won't know 
// the lifetime of the return typw will correspond to which one 
fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(&string1, &string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

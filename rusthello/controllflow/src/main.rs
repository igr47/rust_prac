use std::{os::unix::raw::ino_t, result};

fn main() {
    let age: u32 = 18;
    if age >= 18 {
        println!("You are old enough to drive a car");
    } else {
        println!("Sorry. You are not old enough to drive a car");
    }
    //println!("Hello, world!");

    let number: u32 = 6;
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by either 4, 3, 2");
    }

    let codition: bool = true;
    let number = if codition {5} else {6};
    println!("Number is {}", number);

    // Loop keyword used to run a piece fo code repeatedly untill told to stop
    let mut counter: u32 = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut num: u32 = 3;
    while num != 0 {
        println!("Number {}", num);
        num -= 1;
    }
    println!("HEY");

    let elements = [1, 2, 3, 4];
    for a in elements {
        println!("Elements {}", a);
    }
}

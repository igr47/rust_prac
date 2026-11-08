fn main() {
    println!("Operation {:?}", arithmetic(3, 4, '/'));
    let _val = true;
    let num = 3;

    determinant(_val);
    number_checker(num);
}

fn arithmetic (a: i32, b: i32, op: char) -> i32 {
    match op {
        '+' => a + b,
        '*' => a * b,
        '/' => a / b,
        '-' => a - b,
        '%' => a % b,
        _  => {
            println!("Unkown operator {}", op);
            0
        }
    }
}

fn number_checker(a: i32) {
    match a {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("Some other number"),
    }
}

fn determinant(val: bool) {
    match val {
        true => println!("It's true"),
        false => println!("It's false"),
        //_ => println!("Neither true or false"),
    }
}

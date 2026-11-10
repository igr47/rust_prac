/*struct Numbers {
    number: i32,
}*/

fn main() {
    let my_numbers = vec![10, 20, 30, 40];
    print_numbers(&my_numbers);
    println!("Length of my_numbers {}", print_length(&my_numbers));
}

fn print_numbers(num: &Vec<i32>) {
    for n in num {
        if *n == 30 {
            println!("Thirty");
            continue;
        }
        println!("{:?}", *n);
    }
}

fn print_length(num: &Vec<i32>) -> usize {
    num.len()
}

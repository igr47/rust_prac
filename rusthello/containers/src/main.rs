

fn main() {
    let mut x: Vec<i32> = Vec::new();
    let mut count: i32 = 0;
    while count < 10 {
        //x.push(count);
        count += 1;
        x.push(count);
    }

    for val in x {
        println!("Values {}", val);
    }
}

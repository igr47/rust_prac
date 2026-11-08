fn main() {
    let mut i = 1;
    loop {
        println!("{:?}", i);
        if i == 4 {
            break;
        }
        i += 1;
    }
    println!("Done!");
}

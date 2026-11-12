fn main() {
    let mut name = String::from("Elvis");
    change_name(&mut name);
    change_name(&mut name);
    
}

fn change_name(name: &mut String)  {
    name.push_str(" Kariuki");
    println!("{:?}", name);
}

// Tuple structs
// Declared like 
// Struct Color(i32, i32, i32);
//
// To sue them we do 
// let color = Color(0, 0, 0);

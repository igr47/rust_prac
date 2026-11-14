// Generics: Generic names are defined immediately after the function names  wusing squre brackets.
// fn gen<T>(....parameters) -> return type {}
// If we want to restrict the Generic types to ceratin field like for example those that ca be
// copied or ordered we use Taits
// fn gen<T: we apply trait here>(....parameters) -> return type
//
// Generics can also be used with structs. To do so we add angle brackets  after the struct name and
// add the generic T or more T, U 
// struct Point<T> {
//  x: T,
//  y: T,
// }
// When detininge impelementations using generics we add the <T> after impl and after the Point<t>
// impl <T> Point<t> {
//  Implementation....
// }
//  Sometimes we want some functionality for generics with specific type 
//  impl Point<f64> {
//      ....implement
//  }
//  What above does is that the impplementation in the second impl will only be available with f64 
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point { x: self.x, y: other.y }
    }
}


fn main() {

    let p1 = Point { x: 5, y: 10.5};
    let p2 = Point { x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    let number_list = vec![34, 10, 50, 27, 57, 38];

    let num = get_largest(number_list);
    println!("Largest number number is {}", num);

    let chars = vec!['c', 'd', 'b', 'e', 'k'];
    let largest_char = get_largest(chars);
    println!("Largest charachter is {}", largest_char);
}
// If we didn't add the trait PartialOrd + Copy we could have gotten a compilation error
// since not all generic types can implement the greater that functionality. To restrict to types 
// that allowe for greater that comparison we use traits .
fn get_largest<T: PartialOrd + Copy>(number: Vec<T>) -> T {
    let mut largest_number = number[0];

    for num in number {
        if num > largest_number {
            largest_number = num;
        }
    }
    largest_number
}

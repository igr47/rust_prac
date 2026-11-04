// Every value in rust has a varible which is it's owner
// 2. There can be only one owner at a time 
// 3. When a variable goes out of scope the value is droped.
/*fn main() {
    // s1 is the owner of the value Rust 
    let s1: String = String::from("Rust");
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}", s1, len);

    // Thre can be only one owner at a time. 
    let s2 = &s1;
    println!("{}", s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}*/

// 3. When the owner goes out of scope the value will be dropped. 
/*fn main() {
    let s1: String = String::from("Rust");
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}*/

// Refrences and borrowing 
// Since there can only be one owner at a time if we want to use the value of the vaible with
// another varible we borrow the ownership. That it we reference the original owner of the value.
// We use the & operator to reference or borrow.
/*fn main() {
   let mut x : i32 = 5;
   // If we do:
   // let r = x;
   // We are transfering onweship and; That is x will no longer be the owner of te value 5
   // Instead we reference using & like: let r = &x; and by doing so the r will borrow the value of
   // x which is good for the memmory.
   let r = &mut x;
   *r += 1;
   println!("The value of x {}", r);
   //println!("The value of r {}", r);
   // To do operations on the  borrowed values like addition we need to make the reference mutable.
}*/
// In rust variables are immutable by default. That means that they cannot be changed once set. 
// To change them you prefix the mut keyword before the variable.
// CONSTANTS:
//  Diffrent from imutable variable with:
//      They must be prefixed with const 
//      The name of the variable should be in CAPs
//      There should always be type anotation
//      cannot be decralred mut

const Y: i32 = 10;
const PI: f64 = 3.1459;
fn main() {
    let mut account: BankAccount = BankAccount { owner: "Alice".to_string(), balance: 150.55 };

    account.check_balance();
    account.withdraw(45.0);
    account.check_balance();
    println!("The constant {}", Y);
    println!("The PI {}", PI);
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("Account owned by {} has a balance of {}", self.owner, self.balance);
    }
}

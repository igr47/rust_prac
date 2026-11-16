use std::{thread, time::Duration};
use std::sync::mpsc;
fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you")
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    //let received = rx.recv().unwrap();
    for received in rx {
        println!("{}", received);
    }
}

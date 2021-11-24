use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    let command = args[1].clone();

    println!("{:?}", command);

    if command == "hello" {
        println!("Hello Sachin");
    } else {
        println!("Try again");
    }
}

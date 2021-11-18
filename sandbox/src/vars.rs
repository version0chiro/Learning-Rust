// Variables hold primitive data or reference to data
// Variables are immutable by default
// Rust is block scoped

pub fn run() {
    let name = "Sachin";

    let mut age = 21;
    println!("My name is {0} and I am {1}", name, age);

    age = 22;
    println!("My name is {0} and I am {1}", name, age);

    // Define constant
    const ID: i32 = 001;

    println!("ID: {}", ID);

    // multi variable
    let (my_name, my_age) = ("Sachin", 21);

    println!("{0} is {1}", my_name, my_age);
}

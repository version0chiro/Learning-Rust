// Primitives str = immutable fixed length string somewhere in memory
// String = growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let hello = "hello"; // immutable
    let mut hello_grow = String::from("Hello ");

    println!("{:?}", (hello.len()));

    // pushing a char
    hello_grow.push('W');

    println!("{}", hello_grow);

    // pushing a str
    hello_grow.push_str("orld");

    println!("{}", hello_grow);

    // capacity in bytes
    println!("Capacity :{}", hello_grow.capacity());

    // check if empty
    println!("{}", hello_grow.is_empty());

    // Contains
    println!(
        "Does contain the word 'world' {}",
        hello_grow.contains("World")
    );

    // Replace
    println!("Replaced {}", hello_grow.replace("World", "There"));

    // looping through string using white space

    for word in hello_grow.split_whitespace() {
        println!("{}", word);
    }
    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);

    // println!("{:?}", (hello, hello_grow));
}

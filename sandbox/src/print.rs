pub fn run() {
    // Print to console
    println!("Hello, from print rs file");

    // Print to console with formatting
    println!("{}", 1);

    // Print with multiple play holders
    println!("{} is from {}", "Brad", "Mass");

    // positonal arguments
    println!(
        "{0} is from {1} and  {0} likes to {2}",
        "Brad", "Mass", "code"
    );

    //named arguments
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "Baseball"
    );
    // placee holder trait
    println!("Binary : {:b} Hex: {:x} Octo: {:o}", 10, 10, 10);

    // debuging placeholders
    println!("{:?} ", (12, true, "hello"));

    // basic math
    println!("10+10={}", 10 + 10);
}

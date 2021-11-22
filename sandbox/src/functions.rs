pub fn run() {
    greeting("Hello", "Jane");

    //Bind functions values to vars
    let get_sum = add(10, 10);

    println!("{}", get_sum);

    // Closure
    let n3: i32 = 19;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;

    println!("Csum: {}", add_nums(2, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

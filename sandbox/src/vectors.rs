//  Vectors are resizable arrays
use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // print whole vec
    println!("{:?}", numbers);

    // get single value
    println!("{:?}", numbers[0]);

    // reassign values, we can not add on but change values
    numbers[2] = 20;

    // add to vectors
    numbers.push(5);
    numbers.push(6);

    numbers.pop();

    println!("{:?}", numbers);

    // get vec length
    println!("Vector length:{}", numbers.len());

    // vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // slices
    let slice: &[i32] = &numbers[0..2];

    println!("Slice: {:?}", slice);

    // loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop and mutate
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("{:?}", numbers);
}

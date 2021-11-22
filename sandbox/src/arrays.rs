// Array - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // print whole array
    println!("{:?}", numbers);

    // get single value
    println!("{:?}", numbers[0]);

    // reassign values, we can not add on but change values
    numbers[2] = 20;

    println!("{:?}", numbers);

    // get arart length
    println!("Array length:{}", numbers.len());

    // arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // slices
    let slice: &[i32] = &numbers[0..2];

    println!("Slice: {:?}", slice);
}

/*
Primitive types
Integers : u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (numbers of bits they can take in memory)
Floats : f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays: fixed length
*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.
// however the compiler can infer what type we want to use based on the value and how we use it.

pub fn run() {
    // default is 'i32'
    let x = 1;

    // default is 'f64'
    let y = 2.5;

    //explicit type
    let z: i64 = 2147483648;

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolean

    let is_active: bool = true;

    // Get boolean from expression

    let is_greater: bool = 10 > 5;

    //character
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}

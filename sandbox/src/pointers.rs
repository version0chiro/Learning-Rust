// reference pointerr -> points to a resource in the memeory
pub fn run() {
    // primitive array
    // let arr1 = [1, 2, 3];
    // let arr2 = arr1;

    // with non-primitives if you assign another varible to a piece of data, the first varibale will no longer hold that values, you will need to use a ref to point to the resource
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    // println!("values : {:?}", (arr1, arr2))
    println!("values : {:?}", (&vec1, vec2))
}

fn main() {
    // A ‘vector’ is a dynamic or ‘growable’ array
    let v = vec![1, 2, 3, 4, 5]; // v: Vec<i32>

    let o = vec![0; 10]; // ten zeroes

    // Accessing elements
    // To get the value at a particular index in the vector, use [];
    println!("The third element of v is: {}", v[2]); // The indices count from 0, so the third element is v[2]

    // index MUST has usize type

    let i: usize = 0;
    let j: i32 = 0;

    println!("{}", v[i]); // works
    println!("{}", v[j]); // doesn't works
    // Out-of-bounds Access
    // Accessing index that doesn't exist will return panic!
    println!("{}", v[5]); // will return panic!

    // Use get or get_mut to return None when given index is invalid
    match v.get(5) {
        Some(e) => println!("Item 5 is {}", e),
        None => println!("Sorry, the vector is too short!"),
    }

    // Iterating
    // Iterate verctor elements with for with three different ways:
    for i in &v {
        println!("A reference to {}", i);
    }

    for i in &mut v {
        println!("A mutable reference to {}", i);
    }

    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }

    // Note: You cannot use the vector again once you have iterated by taking ownership of the vector.
}

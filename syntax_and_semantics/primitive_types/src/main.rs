fn main() {
    // Booleans
    // Rust has a built-in boolean type named `bool`. It has two values, `true` and `false
    let o = false;
    println!("o is: {}", o);

    let u: bool = true;
    println!("u is: {}", u);

    let marry_me = true;

    if marry_me {
        println!("Whoah!");
    } else {
        println!("Cry in Javascript");
    }

    // Char
    // The char type represents a single Unicode scalar value.
    // You can create char with a single tick (')
    let bwah = 'x';
    println!("bwah is: {}", bwah);

    let e = "❤❤️️";
    println!("e is: {}", e);

    // Numeric types
    // 1. Signed and Unsigned
    // 2. Fixed and Variable
    // 3. Floating point and Integer
    let x = 42; // x has type i32
    let y = 1.0; // y has type f64

    // List of numeric types
    // 1. i8
    // 2. i16
    // 3. i32
    // 4. i64
    // 5. u8
    // 6. u16
    // 7. u32
    // 8. u64
    // 9. isize
    // 10. usize
    // 11. f32
    // 12. f64

    // Signed and Unsigned
    // To understand the difference, let’s consider a number with four bits of size.
    // A signed, four-bit number would let you store numbers from -8 to +7. Signed numbers use “two’s complement representation”.
    // An unsigned four bit number, since it does not need to store negatives, can store values from 0 to +15.
    // Unsigned types use a u for their category, and signed types use i.
    let p: u32 = 99; // Unsigned

    let p: i32 = 99; // Signed

    /* Arrays */
    // Arrays is immutable by default
    let a = [1, 2, 3]; // a: [i32; 3]
    let mut m = [1, 2, 3]; // m: [i32; 3]

    // Arrays have type [T; N] where T is type and N is the length of array

    let a = [0; 20]; // a: [i32; 20], arrays a will be initialized to `0` with length 20

    // To get the number of elements in array, use `.len()`
    let b = [1, 2, 3];
    println!("b has {} elements", b.len());

    // Access a particular element of an array with subscript notation
    // Rust use zero-index
    let names = ["Teguh", "Suandi"];
    println!("The second name is: {}", names[1]);

    /* Slice */
    // You can use a combo of & and [] to create a slice from various things.
    let a = [0, 1, 2, 3, 4];
    let complete = &a[..]; // A slice containing all elements in a
    let middle = &a[1..4]; // A slice of a: only the elements 1, 2, and 3

    /* Tuples */
    // A tuple is an ordered list of fixed size. Like this:
    let x = (1, "Hello");

    let x: (i32, &str) = (1, "Hello");
    // You can assign one tuple into another, if they have the same contained types and arity. Tuples have the same arity when they have the same length.
    let mut x = (1, 2); // x: (i32, i32)
    let y = (2, 3); // y: (i32, i32)

    //You can access the fields in a tuple through a destructuring let.
    let (x, y, z) = (1, 2, 3);

    println!("x is: {}", x); // will produce `1`

    // Tuple Indexing
    let tuple = (1, 2, 3);
    println!("x is: {}", tuple.0); // will produce `1`

    // Functions
    fn foo(x: i32) -> i32 { x }

    // x is a function pointer to a function that takes i32 as parameter and returns an i32.
    let x: fn(i32) -> i32 = foo;
}

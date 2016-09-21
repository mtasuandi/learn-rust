fn main() {
    /* Variable bindings */
    // "let" is used to introduce a binding
    let x = 9;

    /* Patterns */
    // Left-hand side of "let" statement is a pattern, not a variable name
    // This means e is 9 and r is 7
    let (e, r) = (9, 7);

    /* Type annotations */
    // Basically Rust doesn't require to explicitly type
    // Rust primitive integer types (i8, i16, i32 and i64)
    // i32 means 32-bit signed integer
    let q: i32 = 5;

    /* Mutability */
    // By default, bindings are immutable
    // To make a binding mutable, use "mute"
    let k = 5; // immutable

    let mut k = 5; // mutable

    /* Initializing bindings */
    // Bindings are required to be initialized with a value before you're allowed to use them
    // Rust will not let us use a value that has not been initialized
    
    // This will return a warning unused variable: `m`
    // Because m is not initialized with a value
    let m: i32;
    println!("Hello world");

    // This will return error: use of possibly uninitialized variable: `c`
    // Because c has not been initialized
    let c: i32;
    println!("The value of c is: {}", c);

    // Rust will interpret two curly braces "{}" or moustaches in a string to print as a request to interpolate some sort of value
    // String interpolation is a computer science term that means "stick in the middle of a string".

    /* Scope and shadowing */
    // Variable bindings have a scope - they are constrained to live in a block they were defined in
    // A block is a collection of statements enclosed by { and }
    let g: i32 = 7;
    {
        let h: i32 = 9;
        println!("The value of g is {} and the value of h is {}", g, h);
    }
    println!("The value of g is {} and value of h is {}", g, h); // This won't work because h is not in the same scope
    // The g binding can be accessed, but h will failed because h is inside the g scope

    // Shadowing variable bindings
    let o: i32 = 9;
    {
        println!("{}", o); // Prints 9
        let o = 7;
        println!("{}", o); // Prints 7
    }
    println!("{}", o); // Prints 8
    let o = 99;
    println!("{}", o); // Prints 99

    // Shadowing can also be used to change mutability
    let mut p: i32 = 9; // mutable
    p = 4;
    let p = p; // p is now immutable and is bound to 4

    let w = 4;
    let w = "Hello, my name is Teguh."; // w is now has different type
}

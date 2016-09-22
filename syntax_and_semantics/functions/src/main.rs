/* Functions */
fn main() {
    // There's no "!" like when using println!(); because the print_number is a function, not a macro
    print_number(9);

    print_sum(9, 9);

    add_one(9);
}

// Unlike "let", types of function arguments must be declared.
// function with arguments
fn print_number(n: i32) {
    println!("n is: {}", n);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

// This is a function that adds one to an integer
fn add_one(x: i32) -> i32 {
    x + 1
}

/* Expressions vs. Statements */
// Rust is primarily an expression-based language. There are only two kinds of statements, and everything else is an expression.
// Expressions return a value, and statements do not.
// There are two kinds of statements in Rust: ‘declaration statements’ and ‘expression statements’.

// Declaration statements
// In Rust, using "let" to introduce a binding is not an expression.
// This will produce an error
let i = (let u = 9);
// "let" can only begin a statement, not an expression

// In Rust, assigned value can only have one owner
let mut r = 9;
let k = (r = 7); // k has value "()", not `7`

// Expression statements
// The add_one function will return "()" instead of i32 if there's a semicolon after `x + 1`

/* Early returns */
fn add_one_x(x: i32) -> i32 {
    return x;

    x + 1 // This code never executed
}

// This function works, but is considered poor style!
fn add_one_y(y: i32) -> i32 {
    return y + 1;
}

/* Diverging functions */
fn diverges() -> ! {
    panic!("This function never returns!");
}

// A diverging function can be used as any type:
let w: i32 = diverges();
let w: String = diverges();

/* Function pointers */
// Variable bindings can be pointed to a function
let g: fn(i32) -> i32;

// g is a variable binding which points to a function that takes i32 as an argument and returns an i32.

fn plus_one(o: i32) -> i32 {
    o + 1
}

// Without type inference
let m: fn(i32) -> i32 = plus_one;

// With type inference
let m = plus_one;

// Use `m` to call a function
let nine = m(8);

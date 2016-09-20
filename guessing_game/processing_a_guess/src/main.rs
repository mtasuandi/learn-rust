// In this code, we will take user input and then print the result as output
// We need the "io" library from the standar library
// Rust only imports a few things by default into every program, the prelude (https://doc.rust-lang.org/std/prelude/)
// If it's not in the prelude, you will have to "use" it directly
use std::io;

fn main() {
    println!("Guess the number!"); // Everything with the "!" is calling a macro, not a function (https://doc.rust-lang.org/book/macros.html)

    println!("Please input your guess.");

    // an empty string (https://doc.rust-lang.org/book/strings.html)
    // https://doc.rust-lang.org/book/variable-bindings.html
    // https://doc.rust-lang.org/book/mutability.html
    let mut guess = String::new();

    // https://doc.rust-lang.org/std/io/struct.Stdin.html
    // "&mut guess" is an argument passed to read_line method (https://doc.rust-lang.org/book/method-syntax.html)
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed {}", guess);
}
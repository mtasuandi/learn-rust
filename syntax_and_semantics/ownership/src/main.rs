fn main() {
    let v = vec![1, 2, 3];

    let v2 = v;

    println!("v[0] is: {}", v[0]); // will return error `use of moved value` because value of v was moved to v2

    // Note: Understanding rust ownership is a bit confusing.. haha
    // But here's another rule of Rust ownership..
    // If we define a function that take ownership, and try to use the variable binding after we passed the variable binding to the function
    // it will also return error `use of moved value`
    // Here's the example
    fn takeover(v: Vec<i32>) {
        
    }

    let v2 = vec![1, 2, 3];

    takeover(v);

    println!("v[0] is {}", v[0]);

    // The reason why v cannot be used anymore since the value moved to v2 can be found
    // here https://doc.rust-lang.org/book/ownership.html
}

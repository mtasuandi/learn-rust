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


    // Play with truncate
    let y = vec![9, 8, 7];
    let mut y2 = y;

    y2.truncate(2); // this truncate require the y2 to be mutable, there will be an error if the y2 is not mutable

    println!("{}", y2[0]); // returned 9
    println!("{}", y2[1]); // returned 8
    println!("{}", y2[2]); // returned panic! because the length of vector y2 is 2 - remember zero-indexing

    // No need to be mutable if the sample is only something like this:
    let r = vec![1, 2, 3];
    let r2 = r; // if the r2 is mutable, there will be a warning message if the variable doesn't need to be mutable

    println!("{}", r2[0]); // returned 1

    // Copy
    // All primitive types implement Copy trait, which means when the variable binding copied into another variable binding
    // it will has the same data
    // vector IS NOT a primitive types
    // No error will be returned by this code below
    let v = 1;
    let v2 = v;
    
    println!("v is: {}", v); // returned 1
    println!("v2 is: {}", v2); // returned 1

    let u = 5;
    let _u = double(u);

    println!("{}", _u); // returned 10
    println!("{}", u); // returned 5

    fn double(x: i32) -> i32 {
        x * 2
    }

    // one more example
    let k = true;
    let _k = dibolak(k);

    println!("{}", _k); // returned false
    println!("{}", k); // returned true

    fn dibolak(x: bool) -> bool {
        !x
    }
}

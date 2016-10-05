fn main() {
    fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
        // use tuple, one of Rust primitive types
        let (a, b) = (v1[0], v2[0]);
        a + b
    }
    
    let v1 = vec![1, 2, 3]; // immutable
    let v2 = vec![9, 2, 3]; // immutable
    
    // This is where the borrowing goes
    // The sign `&` is represented as reference
    let answer = foo(&v1, &v2);
    
    println!("{}", v1[0]); // returned 1
    println!("{}", v2[0]); // returned 9
    println!("{}", answer); // returned 10


    // more example
    let x1 = vec![9, 9 ,4];
    let x2 = vec![8, 8, 3];


    // Need to use &
    fn sum_vector(w: &Vec<i32>) -> i32 {
        return w.iter().fold(0, |a, &b| a + b);
    }

    // Need to use &
    fn calculate(t: &Vec<i32>, u: &Vec<i32>) -> i32 {
        let _h1 = sum_vector(t);
        let _h2 = sum_vector(u);

        _h1 + _h2
    }

    let answer = calculate(&x1, &x2);
    println!("{}", answer); // returned 41
    println!("{}", x1[2]); // returned 4
    println!("{}", x2[2]); // returned 3


    // References are immutable, like bindings. This means that inside of foo(), the vectors can’t be changed at all:
    fn mpush(u: &Vec<i32>) {
        u.push(9);
    }

    let _u = vec![3, 4, 6];
    mpush(&_u); // cannot borrow immutable borrowed content `*u` as mutable

    // &mut references
    // You'll need to use astrisks to access the contents of a reference as well.
    let mut _o = 4; {
        let p = &mut _o;
        *p += 1
    };

    println!("{}", _o); // returned 5

    // Here are the rules for borrowing in Rust:
    // First, any borrow must last for a scope no greater than that of the owner.
    // Second, you may have one or the other of these two kinds of borrows, but not both at the same time:
    // one or more references (&T) to a resource
    // exactly one mutable reference (&mut T)

    // Example
    let mut _o = 9;
    let _e = &mut _o;

    *_e += 1;

    println!("{}", _e); // return 10;
    println!("{}", _o); // return `cannot borrow _o as immutable because its also borrowed as mutable`
    println!("{}", &mut _o); // return `cannot borrow _o as mutable more than once at a time`
    let mut _t = 9; {
        let _y = &mut _t;
        *_y += 2;
    }

    println!("{}", _t); // return 11

    // Iterator invalidation
    
}

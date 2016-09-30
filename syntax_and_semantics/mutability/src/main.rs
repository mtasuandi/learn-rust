fn main() {
    // Mutability, the ability to change something, works a bit differently in Rust than in other languages.
    let u = 7;
    u = 9; // error `re-assignment of immutable variable u`

    let mut o = 9;
    o = 10; // no error

    // To create a reference, use &
    &o; // reference to o


    // To use the reference to change the value, need to make it mutable first
    let mut p = 6;
    let l = &mut p; // l is immutable from mutable p

    // If you need to make `l` mutable that reference to mutable variable binding p, you need to use &mut
    let mut p = 4;
    let mut l = &mut p;

    // Interior and Exterior Mutability
    // You may have one or the other of these two kinds of borrows, but not both at the same time:
    // 1. one or more references (&T) to a resource,
    // 2. xactly one mutable reference (&mut T).

    // Example of exterior mutability
    // Arc<T>
    use std::sync::Arc;

    let n = Arc::new(9);
    let m = n.clone();

    // Example of interior mutability
    // RefCell<T>

    use std::cell::RefCell;

    let k = RefCell::new(9);
    let l = k.borrow_mut();


    // Field level mutability
    // Mutability is a property of either a borrow (&mut) or a binding (let mut). This means that, for example, you cannot have a struct with some fields mutable and some immutable:
    struct Score {
        k: i32,
        mut l: i32, // not allowed
    }

    // The mutability of a struct is in its binding:
    struct Score {
        k: i32,
        l: i32,
    }

    let mut a = Score { k: 7, l: 9 };

    a.k = 10;

    let b = Score { k: 9, l: 3 };
    b.k = 12; // error: cannot assign to immutable field `b.x`

    // Use Cell<T> to emulate field-level mutability
    use std::cell:Cell;

    struct Score {
        o: i32,
        p: i32,
    }

    let score = Score { o: 9, p: Cell:new(3) };
    score.p.set(9);

    println!("{}", score.p); // return 9
}

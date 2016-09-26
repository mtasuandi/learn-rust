fn main() {
    // Loop
    loop {
        println!("Loop forever!");
    }

    // While
    let mut r = 5; // mut r: i32
    let mut done = false; // mut done: bool

    while !done {
        r += r - 3;

        println!("{}", r);

        if r % 5 == 0 {
            done = true;
        }
    }

    // Similar with loop but not recommended
    while true {
        println!("Loop forever!");
    }

    // For
    for o in 0..10 { // The upper bound is exclusive (0 -> 9)
        println!("{}", o); // o: i32
    }

    /// ```
    /// for var in expression {
    ///    code
    /// }
    /// ```

    // When you need to keep track of how many times you already looped, you can use the .enumerate() function.

    // On ranges
    for (i, j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i, j);
    }

    /// Output:
    /// i = 0 and j = 5
    /// i = 1 and j = 6
    /// i = 2 and j = 7
    /// i = 3 and j = 8
    /// i = 4 and j = 9

    // On iterators
    let lines = "Hello\nWorld".lines();

    for (linenumber, line) in lines.enumerate() {
        println!("{} : {}", linenumber, line);
    }

    /// Outputs:
    /// 0: Hello
    /// 1: World
    // Rust has two keywords to help us with modifying iteration: break and continue.

    let mut p = 5;
    loop {
        p += p - 3;

        println!("{}", p);

        if p % 5 == 0 { break; }
    }

    for w in 0..10 {
        if w % 2 == 0 { continue; }

        println!("{}", w);
    }

    // Loop labels
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; }
            if y % 2 == 0 { continue 'inner; }

            println!("x: {}, y: {}", x, y);
        }
    }

    /// Outputs:
    /// x: 1, y: 1
    /// x: 1, y: 3
    /// x: 1, y: 5
    /// x: 1, y: 7
    /// x: 1, y: 9
    /// x: 3, y: 1
    /// x: 3, y: 3
    /// x: 3, y: 5
    /// x: 3, y: 7
    /// x: 3, y: 9
    /// x: 5, y: 1
    /// x: 5, y: 3
    /// x: 5, y: 5
    /// x: 5, y: 7
    /// x: 5, y: 9
    /// x: 7, y: 1
    /// x: 7, y: 3
    /// x: 7, y: 5
    /// x: 7, y: 7
    /// x: 7, y: 9
    /// x: 9, y: 1
    /// x: 9, y: 3
    /// x: 9, y: 5
    /// x: 9, y: 7
    /// x: 9, y: 9
}

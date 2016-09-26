fn main() {
    // Line comments are anything after ‘//’ and extend to the end of the line.

    let x = 9; // this is also a line comment.

    println!("{}", x);
    // If you have a long explanation for something, you can put line comments next
    // to each other. Put a space between the // and your comment so that it’s
    // more readable.

    /// Adds one to the number given.
    ///
    /// # Examples
    ///
    /// ```
    /// let five = 5;
    ///
    /// assert_eq!(6, add_one(5));
    /// # fn add_one(x: i32) -> i32 {
    /// #     x + 1
    /// # }
    /// ```
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    assert_eq!(6, add_one(5)); // This compares two values
}

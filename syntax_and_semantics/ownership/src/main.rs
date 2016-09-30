fn main() {
    let v = vec![1, 2, 3];

    let v2 = v;

    println!("v[0] is: {}", v[0]); // will return error `use of moved value` because value of v was moved to v2
}

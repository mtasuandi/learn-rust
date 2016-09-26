fn main() {
    // 1
    let string = "Teguh";

    if string == "Teguh" {
        println!("String is {}!", string);
    }

    // 2
    let number = 9;

    if number == 9 {
        println!("Number is nine!");
    } else {
        println!("Number is not nine!");
    }

    // 3
    let angka = 8;

    if angka == 8 {
        println!("Angka is eight!");
    } else if angka == 9 {
        println!("Angka is nine");
    } else {
        println!("Angka is not eight or nine");
    }

    // 4
    let nomer = 4;

    let int = if nomer == 4 {
        10
    } else {
        15
    }; // y: i32

    // 5
    let nomer = 3;
    let int = if nomer == 3 {13} else {14}; // y:i32
}

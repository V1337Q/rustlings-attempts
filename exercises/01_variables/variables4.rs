// TODO: Fix the compiler error.
fn main() {
    let mut x = 3;
    // x was not mutable.
    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");
}

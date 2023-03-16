fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6; // This will not compile! x is immutable.
    println!("The value of x is {x}");
}

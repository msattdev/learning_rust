fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6; // This line will cause a compile-time error because x is immutable
    println!("The value of x is: {x}");
}

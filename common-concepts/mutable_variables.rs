fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    // immutable
    x = 6;
    println!("The value of x is: {}", x);

    // constant
    const HOURS_IN_SECONDS: u32 = 60 * 60 * 2;
    println!("Value of 2 hours: {}", HOURS_IN_SECONDS);
}
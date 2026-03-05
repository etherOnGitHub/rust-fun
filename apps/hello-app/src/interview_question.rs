pub fn main() {
    // Swap two numbers without using a temporary variable
    let mut a = 5;
    let mut b = 10;
    println!("Before swapping: a = {}, b = {}", a, b);
    a = a + b; // a now holds the sum of a and b
    b = a - b; // b now holds the original value of a
    a = a - b; // a now holds the original value of b
    println!("After swapping: a = {}, b = {}", a, b);

    // You can also use XOR to swap two numbers without a temporary variable
    let mut x = 5;
    let mut y = 10;
    println!("Before swapping: x = {}, y = {}", x, y);
    x = x ^ y; // x now holds the result of x XOR y
    y = x ^ y; // y now holds the original value of x
    x = x ^ y; // x now holds the original value of y
    println!("After swapping: x = {}, y = {}", x, y);
}
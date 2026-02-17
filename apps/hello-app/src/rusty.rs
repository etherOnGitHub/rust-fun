pub fn main() {
    println!("This is the rusty module's main function!");

    let x:bool = true;

    if x {
        println!("x is true");
    } else {
        println!("x is false");
    }

    let y:i8 = 6;
    if y == 4 {
        println!("y is 4");
    } else if y > 4 {
        println!("y is greater than 4");
        } else {
        println!("y is not 4");
    }
}
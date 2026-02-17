pub fn slicing() {
    println!("Slicing function called");
    let vec = vec![1, 2, 3, 4, 5];
    let int = &vec[..];
    let string: &[&str] = &["Hello", "World"];

    println!("The vec slice is: {:?}", vec);
    println!("The int slice is: {:?}", int);
    println!("The string slice is: {:?}", string);
    assert_eq!(string.len(), 2);
    assert!(!string.is_empty());

    let string = "Hello, Rust!";
    let string2 = "Nice
     to 
     meet 
     you!";
    let concat = format!("{} {}", string, string2);
    println!("The concatenated string is: {}", concat);

    // concat multi string + with string literal
    let stringy1 = "Hello, 
    Program is running! ";
    let stringy2 = "Rust!";
    let concat_string = stringy1.to_string() + stringy2;
    println!("The concatenated string is: {}", concat_string);

    let mut stringy3 = "bruh".to_string();
    stringy3.push_str(" moment");
    println!("The string after push_str is: {}", stringy3);
}

pub fn main() {
    slicing();
}
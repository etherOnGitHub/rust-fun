use core::greeting::greeting;
use core::greeting::reverser;
use validation::validation::validate_name;

fn main() {
    let name = "etherchild";

    if let Err(error) = validate_name(name) {
        println!("Validation error: {}", error);
        return;
    } else {
        println!("Name is valid.");
    }

    let message =  greeting("etherchild");
    println!("{}", message);

    
    // Creates a string and reverses it.
    // The `chars()` method converts the string into an iterator of characters.
    // The `rev()` method reverses the order of the characters.
    // The `collect()` gathers them back into a `String`.
    
    let reversed = reverser(name);
    println!("Original: {}", name);
    println!("Reversed: {}", reversed);
}

use core::greeting::greeting;
use core::greeting::reverser;
use core::greeting::even_numbers_total;
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

    
    /* Creates a string and reverses it.
    The `chars()` method converts the string into an iterator of characters.
    The `rev()` method reverses the order of the characters.
    The `collect()` gathers them back into a `String`. */
    
    let reversed = reverser(name);
    println!("Original: {}", name);
    println!("Reversed: {}", reversed);

    // Creates an array of numbers and passes it into a function to use modulo to count even numbers and add them together.
    let numbers = [1, 2, 3, 4, 5, 6];
    let total = even_numbers_total(&numbers);
    println!("Total of even numbers: {}", total);
}

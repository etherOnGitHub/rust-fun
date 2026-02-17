use std::mem;
mod rusty;
mod bruh;
mod listen;
mod ok;

use core::greeting::greeting;
use core::greeting::reverser;
use core::greeting::even_numbers_total;
use validation::validation::validate_name;
use messingaround::alison::hiworld;
use messingaround::alison::bool;

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

    hiworld();
    println!("Boolean value: {}", bool());

    let c = "c";
    let number = 42;
    println!("I got this letter here: {} but I also got this stupid ahh number: {}", c, number);
    let scope = "";
    println!("Size of bool: {} bytes", mem::size_of_val(&bool));
    {
        let scope = "This is a new scope";
        println!("Inside the scope: {}", scope);
    }
    if scope == "This is a new scope" {
        println!("Scope is still accessible here: {}", scope);
    } else {
        println!("Scope is not accessible here.");
    }

    let n1 = 10;
    let n2 = 20;
    {
        let n3 = n1 + n2;
        println!("The sum of n1 and n2 is: {}", n3);
    }
    let n3 = n1 * n2;
    println!("The product of n1 and n2 is: {}", n3);

    rusty::main();
    bruh::main();
    listen::main();
    ok::main();
}

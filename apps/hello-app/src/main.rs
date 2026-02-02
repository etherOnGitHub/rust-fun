use core::greeting::greeting;
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
}

use core::greeting::greeting;
use validation::validation::validate_name;

fn main() {
    let message =  greeting("etherchild");

    if let Err(error) = validate_name(&message) {
        println!("Validation error: {}", error);
        return;
    }

    println!("{}", message);
}

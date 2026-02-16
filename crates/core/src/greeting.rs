

// This function takes a string slice `name` and returns a new `String` that is a greeting message.
pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

// This function takes a string slice `name` and returns a new `String` that is the reverse of the input.
pub fn reverser(name: &str) -> String {
    println!("Reverser function called");
    name.chars().rev().collect::<String>()
}

pub fn even_numbers_total(arr: &[i32]) -> i32 {
    let mut total = 0;

    for num in arr {
        if num % 2 == 0 {
            total += num;
        }
    }

    total
}

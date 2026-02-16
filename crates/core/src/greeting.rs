

// This function takes a string slice `name` and returns a new `String` that is a greeting message.
pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

// This function takes a string slice `name` and returns a new `String` that is the reverse of the input.
pub fn reverser(name: &str) -> String {
    println!("Reverser function called");
    name.chars().rev().collect::<String>()
}

// This function takes a slice of integers and returns the total of the even numbers in the slice.
pub fn even_numbers_total(arr: &[i32]) -> i32 {
    // Declare a mutable variable `total` to keep track of the sum of even numbers.
    let mut total = 0;
    // Iterate through each number in the array.
    for num in arr {
        // Check if the number is even using the modulo operator.
        if num % 2 == 0 {
            // If the number is even, add it to the total.
            total += num;
        }
    }
    // Return the total of even numbers.
    total
}

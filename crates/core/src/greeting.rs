pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn reverser(name: &str) -> String {
    name.chars().rev().collect::<String>()
}
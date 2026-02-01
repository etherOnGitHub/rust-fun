pub fn validate_name(name: &str) -> Result<(), String> {
    if name.trim().is_empty() {
        Err("Name cannot be empty".to_string())
    } else {
        Ok(())
    }
}
pub fn validate_name(name: &str) -> Result<(), String> {
    if name.trim().is_empty() {
        Err("Name cannot be empty".to_string())
    } else {
        Ok(())
    }
}

// Run tests with `cargo test` in the `validation` crate directory. This will execute the tests defined in the `tests` module.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_name() {
        assert!(validate_name("Alice").is_ok());
        assert!(validate_name("  Bob  ").is_ok());
        assert!(validate_name("").is_err());
        assert!(validate_name("   ").is_err());
    }
}
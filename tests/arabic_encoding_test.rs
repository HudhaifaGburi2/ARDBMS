#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_arabic() {
        let text = "مرحبا بالعالم";
        assert!(encoding::validate_arabic(text));
    }

    #[test]
    fn test_invalid_mixed_chars() {
        let text = "Hello مرحبا";
        assert!(!encoding::validate_arabic(text));
    }
}

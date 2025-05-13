// tests/arabic_encoding_test.rs
use ardbms::arabic::encoding;  // Assuming your crate is named "ardbms"

#[test]
fn test_arabic_validation() {
    assert!(encoding::validate_arabic("مرحبا"));
    assert!(!encoding::validate_arabic("Hello 123"));
}

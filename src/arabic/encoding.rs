pub mod encoding {
    pub fn validate_arabic(text: &str) -> bool {
        text.chars().all(|c| c.is_arabic())
    }

    pub trait ArabicChar {
        fn is_arabic(&self) -> bool;
    }

    impl ArabicChar for char {
        fn is_arabic(&self) -> bool {
            matches!(*self as u32,
                0x0600..=0x06FF |
                0x0750..=0x077F |
                0x08A0..=0x08FF |
                0xFB50..=0xFDFF |
                0xFE70..=0xFEFF
            )
        }
    }
}

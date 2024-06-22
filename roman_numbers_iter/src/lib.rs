pub mod roman_numbers_iter {
    #[derive(Debug, PartialEq)]
    pub struct RomanNumber(pub u32);

    impl RomanNumber {
        pub fn from(value: u32) -> Self {
            RomanNumber(value)
        }
    }

    impl Iterator for RomanNumber {
        type Item = RomanNumber;

        fn next(&mut self) -> Option<Self::Item> {
            self.0 += 1;
            Some(RomanNumber(self.0 - 1))
        }
    }
}
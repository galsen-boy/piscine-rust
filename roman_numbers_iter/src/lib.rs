#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    IV, 
    V,
    IX, 
    X,
    XL, 
    L,
    XC, 
    C,
    CD, 
    D,
    CM, 
    M,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);
impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        if num==0 {
           return RomanNumber([RomanDigit::Nulla].to_vec())
        }
        let mut digits = Vec::new();
        let values = [(1000, vec![M]), (900, vec![C, M]), (500, vec![D]), (400, vec![C, D]), (100, vec![C]),
                      (90, vec![X, C]), (50, vec![L]), (40, vec![X, L]), (10, vec![X]),
                      (9, vec![I, X]), (5, vec![V]), (4, vec![I, V]), (1, vec![I])];
        for &(value, ref digits_seq) in &values {
            while num >= value {
                digits.extend(digits_seq);
                num -= value;
            }
        }
        RomanNumber(digits)
    }
}
use crate::RomanDigit::*;
pub fn from_roman_number(roman: RomanNumber) -> u32 {
    let mut total = 0;
    let mut last_value = 0;
    for digit in &roman.0 {
        let value = match *digit {
            RomanDigit::Nulla => 0,
            RomanDigit::I => 1,
            RomanDigit::IV => 4,
            RomanDigit::V => 5,
            RomanDigit::IX => 9,
            RomanDigit::X => 10,
            RomanDigit::XL => 40,
            RomanDigit::L => 50,
            RomanDigit::XC => 90,
            RomanDigit::C => 100,
            RomanDigit::CD => 400,
            RomanDigit::D => 500,
            RomanDigit::CM => 900,
            RomanDigit::M => 1000,
        };
        if value > last_value {
            total += value - 2 * last_value;
        } else {
            total += value;
        }
        last_value = value;
    }
    total
}
impl Iterator for RomanNumber {
    type Item = RomanNumber;
    fn next(&mut self) -> Option<Self::Item> {
        Some(RomanNumber::from(from_roman_number(self.clone())+1))
    }
}
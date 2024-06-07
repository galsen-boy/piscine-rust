pub fn number_logic(num: u32) -> bool {
    let num_str = num.to_string();
    let num_len = num_str.len() as u32;
    let mut sum = 0;
    for i in num_str.chars() {
        let digit = i.to_digit(10).unwrap();
        sum += digit.pow(num_len);
    }
    sum == num
}
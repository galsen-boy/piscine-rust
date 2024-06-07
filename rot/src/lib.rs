pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                let base = if c.is_lowercase() { b'a' } else { b'A' } as i8;
                (((c as i8 - base + key) % 26 + 26) % 26 + base) as u8 as char
            } else {
                c
            }
        })
        .collect()
}
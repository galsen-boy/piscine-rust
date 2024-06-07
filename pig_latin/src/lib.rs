pub fn pig_latin(text: &str) -> String {
    for (i, c) in text.chars().enumerate() {
        if "aeiou".contains(c) {
            if c == 'u' && i >= 2 && text[i - 2..].starts_with("squ") {
                return text[i + 1..].to_string() + &text[..i + 1] + "ay";
            }
            return text[i..].to_string() + &text[..i] + "ay";
        }
    }
    text.to_string() + "ay"
}
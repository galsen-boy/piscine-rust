pub fn is_anagram(s1: &str, s2: &str) -> bool {

    let mut s1_chars: Vec<char> = s1.to_lowercase().chars().collect();
    let mut s2_chars: Vec<char> = s2.to_lowercase().chars().collect();

    s1_chars.sort();
    s2_chars.sort();

    s1_chars == s2_chars
}&
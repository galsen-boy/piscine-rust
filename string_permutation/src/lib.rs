use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut first_str = HashMap::new();
    let mut second_str = HashMap::new();

    for ch in s1.chars() {
        first_str
            .entry(ch)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    for ch in s2.chars() {
        second_str
            .entry(ch)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    first_str == second_str
}
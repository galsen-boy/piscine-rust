
pub fn talking(text: &str) -> &str {
    match text {
        t if t.trim() == "" => "Just say something!",
        t if t.chars().any(|c| c.is_alphabetic())
            && t.to_uppercase() == text
            && t.ends_with('?') =>
        {
            "Quiet, I am thinking!"
        }
        t if t.chars().any(|c| c.is_alphabetic()) && t.to_uppercase() == text => {
            "There is no need to yell, calm down!"
        }
        t if t.ends_with('?') => "Sure.",
        _ => "Interesting",
    }
}
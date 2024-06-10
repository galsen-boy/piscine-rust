pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    Box::new(s.split_whitespace()
        .map(|mut s| {
            let mut mul = 1;
            if s.ends_with('k') {
                mul = 1000;
                s = &s[..s.len()-1];
            }
            let mut n: f64 = s.parse().expect("Could not parse the string");
            n *= mul as f64;
            n as u32
        })
        .collect())
}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}
use std::collections::HashMap;
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut min = 0;
    for (_, val) in h.iter(){
        if *val > min {
            min = *val;
        }
    }
    min
}
pub fn to_url(s: &str) -> String {
    let mut url = String::new();
    for i in s.chars() {
        if i == ' ' {
            url.push_str("%20");
        }else {
            url.push_str(i);
        }
    }
    url
}
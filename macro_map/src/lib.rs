#[macro_export]
macro_rules! hash_map {
    // Match empty input and return an empty HashMap
    () => {
        {
            std::collections::HashMap::new()
        }
    };
    
    // Match single key-value pair without trailing comma
    ($key:expr => $value:expr) => {
        {
            let mut map = std::collections::HashMap::new();
            map.insert($key, $value);
            map
        }
    };
    
    // Match multiple key-value pairs, allowing trailing comma
    ($($key:expr => $value:expr),+ $(,)?) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )+
            map
        }
    };
}
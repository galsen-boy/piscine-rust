pub fn spell(n: u64) -> String {
    let units = ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let teens = ["", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
    let tens = ["", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
    let thousands = ["", "thousand", "million"];
    let hundreds = ["", "hundred"];

    if n == 0 {
        return "zero".to_string();
    }

    let mut result = String::new();
    let mut n = n;
    let mut i = 0;

    while n > 0 {
        let chunk = n % 1000;
        n /= 1000;
        let mut chunk_str = String::new();

        if chunk >= 100 {
            chunk_str.push_str(&units[(chunk / 100) as usize]);
            chunk_str.push(' ');
            chunk_str.push_str(hundreds[1]);
            chunk %= 100;
            if chunk > 0 {
                chunk_str.push(' ');
            }
        }

        if chunk >= 11 && chunk <= 19 {
            chunk_str.push_str(&teens[(chunk - 10) as usize]);
        } else if chunk >= 20 {
            chunk_str.push_str(&tens[(chunk / 10) as usize]);
            chunk %= 10;
            if chunk > 0 {
                chunk_str.push('-');
            }
        }

        if chunk > 0 && chunk < 10 {
            chunk_str.push_str(&units[chunk as usize]);
        }

        if i > 0 && chunk > 0 {
            chunk_str.push(' ');
            chunk_str.push_str(&thousands[i]);
        }

        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(&chunk_str);

        i += 1;
    }

    result.trim().to_string()
}
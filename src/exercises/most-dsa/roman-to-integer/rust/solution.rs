use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let roman: HashMap<char, i32> = [
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]
        .iter()
        .cloned()
        .collect();

        let chars: Vec<char> = s.chars().collect();
        let mut res = 0;
        for i in 0..chars.len() {
            let curr = roman[&chars[i]];
            if i + 1 < chars.len() && curr < roman[&chars[i + 1]] {
                res -= curr;
            } else {
                res += curr;
            }
        }
        res
    }
}

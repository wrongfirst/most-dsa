impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut string_stack: Vec<String> = Vec::new();
        let mut count_stack: Vec<i32> = Vec::new();
        let mut cur = String::new();
        let mut k: i32 = 0;

        for c in s.chars() {
            if c.is_ascii_digit() {
                k = k * 10 + c.to_digit(10).unwrap() as i32;
            } else if c == '[' {
                string_stack.push(cur.clone());
                count_stack.push(k);
                cur = String::new();
                k = 0;
            } else if c == ']' {
                let temp = cur.clone();
                cur = string_stack.pop().unwrap();
                let count = count_stack.pop().unwrap();
                for _ in 0..count {
                    cur.push_str(&temp);
                }
            } else {
                cur.push(c);
            }
        }

        cur
    }
}

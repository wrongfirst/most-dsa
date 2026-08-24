impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut result = String::new();
        let mut n = column_number;

        while n > 0 {
            n -= 1;
            let offset = (n % 26) as u8;
            result.push((b'A' + offset) as char);
            n /= 26;
        }

        result.chars().rev().collect()
    }
}

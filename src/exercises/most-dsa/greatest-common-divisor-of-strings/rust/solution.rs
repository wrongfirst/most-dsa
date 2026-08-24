impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        fn gcd(a: usize, b: usize) -> usize {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }

        let g = gcd(str1.len(), str2.len());
        let str1_bytes = str1.as_bytes();
        let str2_bytes = str2.as_bytes();

        let all_match_str1 = str1_bytes.iter().enumerate().all(|(i, &c)| c == str1_bytes[i % g]);
        let all_match_str2 = str2_bytes.iter().enumerate().all(|(i, &c)| c == str1_bytes[i % g]);

        if all_match_str1 && all_match_str2 {
            str1[..g].to_string()
        } else {
            String::new()
        }
    }
}

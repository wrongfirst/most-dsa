impl Solution {
    pub fn reorganize_string(s: String) -> String {
        if s.is_empty() {
            return String::new();
        }
        let mut freq = [0i32; 26];
        for c in s.chars() {
            freq[(c as usize) - ('a' as usize)] += 1;
        }

        let max_idx = freq.iter().enumerate().max_by_key(|&(_, &v)| v).unwrap().0;
        let max_freq = freq[max_idx];

        if max_freq > ((s.len() + 1) / 2) as i32 {
            return String::new();
        }

        let mut res = vec![' '; s.len()];
        let mut idx = 0;
        let max_char = (max_idx as u8 + b'a') as char;

        // Place the most frequent character first at even indices
        while freq[max_idx] > 0 {
            res[idx] = max_char;
            idx += 2;
            freq[max_idx] -= 1;
        }

        // Place remaining characters
        for i in 0..26 {
            while freq[i] > 0 {
                if idx >= s.len() {
                    idx = 1;
                }
                res[idx] = (i as u8 + b'a') as char;
                idx += 2;
                freq[i] -= 1;
            }
        }

        res.into_iter().collect()
    }
}

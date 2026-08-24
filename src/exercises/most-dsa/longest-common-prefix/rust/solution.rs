impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        let mut prefix: Vec<char> = strs[0].chars().collect();

        for i in 1..strs.len() {
            let s: Vec<char> = strs[i].chars().collect();
            let mut j = 0;
            while j < prefix.len() && j < s.len() {
                if prefix[j] != s[j] {
                    break;
                }
                j += 1;
            }
            prefix.truncate(j);
        }

        prefix.into_iter().collect()
    }
}

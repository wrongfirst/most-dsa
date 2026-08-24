impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let chars1: Vec<char> = word1.chars().collect();
        let chars2: Vec<char> = word2.chars().collect();
        let n = chars1.len();
        let m = chars2.len();
        let mut result = String::new();

        for i in 0..n.max(m) {
            if i < n {
                result.push(chars1[i]);
            }
            if i < m {
                result.push(chars2[i]);
            }
        }

        result
    }
}

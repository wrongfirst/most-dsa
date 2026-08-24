use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let word_set: HashSet<String> = word_dict.into_iter().collect();
        let n = s.len();
        let mut dp: Vec<Vec<String>> = vec![Vec::new(); n + 1];
        dp[0].push(String::new());

        for i in 1..=n {
            for j in 0..i {
                let word = &s[j..i];
                if word_set.contains(word) {
                    let sentences: Vec<String> = dp[j]
                        .iter()
                        .map(|sentence| {
                            if sentence.is_empty() {
                                word.to_string()
                            } else {
                                format!("{} {}", sentence, word)
                            }
                        })
                        .collect();
                    dp[i].extend(sentences);
                }
            }
        }

        dp[n].clone()
    }
}

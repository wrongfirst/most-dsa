struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: Default::default(),
            is_word: false,
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn add_word(&mut self, word: &str) {
        let mut curr = &mut self.root;
        for c in word.chars() {
            let idx = (c as u8 - b'a') as usize;
            curr = curr.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        curr.is_word = true;
    }
}

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let mut trie = Trie::new();
        for word in &dictionary {
            trie.add_word(word);
        }

        let n = s.len();
        let chars: Vec<char> = s.chars().collect();
        let mut dp = vec![0; n + 1];

        for i in (0..n).rev() {
            dp[i] = 1 + dp[i + 1];
            let mut curr = &trie.root;

            for j in i..n {
                let idx = (chars[j] as u8 - b'a') as usize;
                if curr.children[idx].is_none() {
                    break;
                }
                curr = curr.children[idx].as_ref().unwrap();
                if curr.is_word {
                    dp[i] = dp[i].min(dp[j + 1]);
                }
            }
        }

        dp[0]
    }
}

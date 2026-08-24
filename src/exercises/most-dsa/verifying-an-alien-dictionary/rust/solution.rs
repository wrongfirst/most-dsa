impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut order_index = [0; 26];
        for (i, c) in order.chars().enumerate() {
            order_index[(c as usize) - ('a' as usize)] = i;
        }

        for i in 0..words.len() - 1 {
            let w1: Vec<char> = words[i].chars().collect();
            let w2: Vec<char> = words[i + 1].chars().collect();

            let mut j = 0;
            while j < w1.len() {
                if j == w2.len() {
                    return false;
                }
                if w1[j] != w2[j] {
                    let idx1 = (w1[j] as usize) - ('a' as usize);
                    let idx2 = (w2[j] as usize) - ('a' as usize);
                    if order_index[idx1] > order_index[idx2] {
                        return false;
                    }
                    break;
                }
                j += 1;
            }
        }

        true
    }
}

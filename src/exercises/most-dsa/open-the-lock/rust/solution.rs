use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        if target == "0000" {
            return 0;
        }

        let mut visit: HashSet<String> = deadends.into_iter().collect();
        if visit.contains("0000") {
            return -1;
        }

        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back("0000".to_string());
        visit.insert("0000".to_string());
        let mut steps = 0;

        while !queue.is_empty() {
            steps += 1;
            let size = queue.len();
            for _ in 0..size {
                let lock = queue.pop_front().unwrap();
                let lock_bytes = lock.as_bytes();

                for i in 0..4 {
                    for j in &[1i32, -1i32] {
                        let digit = ((lock_bytes[i] - b'0') as i32 + j + 10) % 10;
                        let mut next_lock = lock.clone().into_bytes();
                        next_lock[i] = (digit as u8) + b'0';
                        let next_lock_str = String::from_utf8(next_lock).unwrap();

                        if visit.contains(&next_lock_str) {
                            continue;
                        }
                        if next_lock_str == target {
                            return steps;
                        }
                        queue.push_back(next_lock_str.clone());
                        visit.insert(next_lock_str);
                    }
                }
            }
        }
        -1
    }
}

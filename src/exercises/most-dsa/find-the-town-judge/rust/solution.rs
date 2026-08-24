use std::collections::HashMap;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut delta: HashMap<i32, i32> = HashMap::new();

        for t in &trust {
            let src = t[0];
            let dst = t[1];
            *delta.entry(src).or_insert(0) -= 1;
            *delta.entry(dst).or_insert(0) += 1;
        }

        for i in 1..=n {
            if *delta.get(&i).unwrap_or(&0) == n - 1 {
                return i;
            }
        }

        -1
    }
}

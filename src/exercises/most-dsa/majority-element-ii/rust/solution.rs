use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut count: HashMap<i32, i32> = HashMap::new();

        for &num in &nums {
            *count.entry(num).or_insert(0) += 1;

            if count.len() <= 2 {
                continue;
            }

            let mut new_count: HashMap<i32, i32> = HashMap::new();
            for (&num, &c) in &count {
                if c > 1 {
                    new_count.insert(num, c - 1);
                }
            }
            count = new_count;
        }

        let threshold = (nums.len() / 3) as i32;
        let mut res = Vec::new();

        for &num in count.keys() {
            if nums.iter().filter(|&&x| x == num).count() as i32 > threshold {
                res.push(num);
            }
        }

        res
    }
}

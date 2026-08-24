use std::collections::HashMap;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut dp: HashMap<i32, i32> = HashMap::new();
        dp.insert(target, 1);

        for total in (1..=target).rev() {
            for &num in &nums {
                if total < num {
                    break;
                }
                let cur_val = *dp.get(&total).unwrap_or(&0);
                let prev_val = *dp.get(&(total - num)).unwrap_or(&0);
                dp.insert(total - num, cur_val + prev_val);
            }
        }
        *dp.get(&0).unwrap_or(&0)
    }
}

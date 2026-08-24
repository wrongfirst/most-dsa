use std::collections::HashSet;

impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let stone_sum: i32 = stones.iter().sum();
        let target = stone_sum / 2;
        let mut dp: HashSet<i32> = HashSet::new();
        dp.insert(0);

        for stone in stones {
            let mut new_dp = dp.clone();
            for &val in &dp {
                if val + stone == target {
                    return stone_sum - 2 * target;
                }
                if val + stone < target {
                    new_dp.insert(val + stone);
                }
            }
            dp = new_dp;
        }

        stone_sum - 2 * dp.iter().max().unwrap()
    }
}

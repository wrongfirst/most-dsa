impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let total: i32 = nums.iter().sum();
        if total % k != 0 {
            return false;
        }

        let target = total / k;
        let n = nums.len();
        let num_states = 1 << n;
        let mut dp = vec![-1i32; num_states];
        dp[0] = 0;

        for mask in 0..num_states {
            if dp[mask] == -1 {
                continue;
            }
            for i in 0..n {
                if (mask & (1 << i)) == 0 && dp[mask] + nums[i] <= target {
                    dp[mask | (1 << i)] = (dp[mask] + nums[i]) % target;
                }
            }
        }

        dp[num_states - 1] == 0
    }
}

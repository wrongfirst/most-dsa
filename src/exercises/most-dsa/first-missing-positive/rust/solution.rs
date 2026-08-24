impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len() as i32;

        // Replace negative numbers with 0
        for i in 0..nums.len() {
            if nums[i] < 0 {
                nums[i] = 0;
            }
        }

        // Mark numbers that exist
        for i in 0..nums.len() {
            let val = nums[i].abs();
            if val >= 1 && val <= n {
                let idx = (val - 1) as usize;
                if nums[idx] > 0 {
                    nums[idx] *= -1;
                } else if nums[idx] == 0 {
                    nums[idx] = -(n + 1);
                }
            }
        }

        // Find first missing positive
        for i in 1..=n {
            let idx = (i - 1) as usize;
            if nums[idx] >= 0 {
                return i;
            }
        }

        n + 1
    }
}

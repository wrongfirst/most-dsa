impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut total: i64 = 0;
        let mut res = i32::MAX;
        let target = target as i64;

        for r in 0..nums.len() {
            total += nums[r] as i64;
            while total >= target {
                res = res.min((r - l + 1) as i32);
                total -= nums[l] as i64;
                l += 1;
            }
        }

        if res == i32::MAX { 0 } else { res }
    }
}

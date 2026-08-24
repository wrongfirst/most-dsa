impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l: i32 = 0;
        let mut r: i32 = nums.len() as i32;

        while l < r {
            let m = l + (r - l) / 2;
            if nums[m as usize] >= target {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l
    }
}

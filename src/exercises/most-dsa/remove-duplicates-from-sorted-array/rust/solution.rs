impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut l = 1;
        for r in 1..nums.len() {
            if nums[r] != nums[r - 1] {
                nums[l] = nums[r];
                l += 1;
            }
        }
        l as i32
    }
}

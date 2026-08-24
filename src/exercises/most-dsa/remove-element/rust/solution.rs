impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        let mut n = nums.len();
        while i < n {
            if nums[i] == val {
                n -= 1;
                nums[i] = nums[n];
            } else {
                i += 1;
            }
        }
        n as i32
    }
}

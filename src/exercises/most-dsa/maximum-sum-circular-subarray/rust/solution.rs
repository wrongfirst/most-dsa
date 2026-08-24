impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut glob_max = nums[0];
        let mut glob_min = nums[0];
        let mut cur_max = 0;
        let mut cur_min = 0;
        let mut total = 0;

        for num in nums {
            cur_max = cur_max.max(0) + num;
            cur_min = cur_min.min(0) + num;
            total += num;
            glob_max = glob_max.max(cur_max);
            glob_min = glob_min.min(cur_min);
        }

        if glob_max > 0 {
            glob_max.max(total - glob_min)
        } else {
            glob_max
        }
    }
}

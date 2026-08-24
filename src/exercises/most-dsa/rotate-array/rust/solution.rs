impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        if n == 0 {
            return;
        }
        let k = (k as usize) % n;
        if k == 0 {
            return;
        }

        fn reverse(nums: &mut Vec<i32>, mut l: usize, mut r: usize) {
            while l < r {
                nums.swap(l, r);
                l += 1;
                r -= 1;
            }
        }

        reverse(nums, 0, n - 1);
        reverse(nums, 0, k - 1);
        reverse(nums, k, n - 1);
    }
}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut last = (m + n - 1) as i32;
        let mut i = m as i32 - 1;
        let mut j = n as i32 - 1;

        while j >= 0 {
            if i >= 0 && nums1[i as usize] > nums2[j as usize] {
                nums1[last as usize] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[last as usize] = nums2[j as usize];
                j -= 1;
            }
            last -= 1;
        }
    }
}

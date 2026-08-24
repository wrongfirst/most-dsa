impl Solution {
    pub fn find_in_mountain_array(target: i32, mountain_arr: &dyn MountainArray) -> i32 {
        let length = mountain_arr.length();

        // Find Peak
        let mut l = 1;
        let mut r = length - 2;
        let mut peak = 0;
        while l <= r {
            let m = (l + r) / 2;
            let left = mountain_arr.get(m - 1);
            let mid = mountain_arr.get(m);
            let right = mountain_arr.get(m + 1);
            if left < mid && mid < right {
                l = m + 1;
            } else if left > mid && mid > right {
                r = m - 1;
            } else {
                peak = m;
                break;
            }
        }

        // Search left portion
        l = 0;
        r = peak - 1;
        while l <= r {
            let m = (l + r) / 2;
            let val = mountain_arr.get(m);
            if val < target {
                l = m + 1;
            } else if val > target {
                r = m - 1;
            } else {
                return m;
            }
        }

        // Search right portion
        l = peak;
        r = length - 1;
        while l <= r {
            let m = (l + r) / 2;
            let val = mountain_arr.get(m);
            if val > target {
                l = m + 1;
            } else if val < target {
                r = m - 1;
            } else {
                return m;
            }
        }

        -1
    }
}

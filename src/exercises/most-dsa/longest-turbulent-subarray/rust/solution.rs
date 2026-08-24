impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut res = 0;
        let mut cnt = 0;
        let mut sign: i32 = -1; // -1 = unset, 0 = less than, 1 = greater than

        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                cnt = if sign == 0 { cnt + 1 } else { 1 };
                sign = 1;
            } else if arr[i] < arr[i + 1] {
                cnt = if sign == 1 { cnt + 1 } else { 1 };
                sign = 0;
            } else {
                cnt = 0;
                sign = -1;
            }
            res = res.max(cnt);
        }

        res + 1
    }
}

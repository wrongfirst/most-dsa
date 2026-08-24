impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let mut l = 0;
        let mut r = arr.len() - k;

        while l < r {
            let m = (l + r) / 2;
            if x - arr[m] > arr[m + k] - x {
                l = m + 1;
            } else {
                r = m;
            }
        }

        arr[l..l + k].to_vec()
    }
}

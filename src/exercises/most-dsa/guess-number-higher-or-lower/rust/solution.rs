impl Solution {
    pub unsafe fn guess_number(n: i64) -> i64 {
        let mut l: i64 = 1;
        let mut r: i64 = n;

        loop {
            let m = l + (r - l) / 2;
            let res = guess(m);
            if res > 0 {
                l = m + 1;
            } else if res < 0 {
                r = m - 1;
            } else {
                return m;
            }
        }
    }
}

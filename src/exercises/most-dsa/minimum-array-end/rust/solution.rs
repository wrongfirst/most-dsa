impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let x = x as i64;
        let n = n as i64;
        let mut res = x;
        let mut i_x: i64 = 1;
        let mut i_n: i64 = 1;

        while i_n <= n - 1 {
            if (i_x & x) == 0 {
                if (i_n & (n - 1)) != 0 {
                    res |= i_x;
                }
                i_n <<= 1;
            }
            i_x <<= 1;
        }

        res
    }
}

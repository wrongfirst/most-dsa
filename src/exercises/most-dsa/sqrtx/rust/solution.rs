impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let mut r = x as i64;
        let x_i64 = x as i64;
        while r * r > x_i64 {
            r = (r + x_i64 / r) >> 1;
        }
        r as i32
    }
}

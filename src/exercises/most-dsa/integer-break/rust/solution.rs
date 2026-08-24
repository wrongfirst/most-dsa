impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n <= 3 {
            return n - 1;
        }

        let quotient = n / 3;
        let remainder = n % 3;

        let base: i64 = 3;
        let res = base.pow(quotient as u32) as i32;

        if remainder == 1 {
            (res / 3) * 4
        } else if remainder == 2 {
            res * 2
        } else {
            res
        }
    }
}

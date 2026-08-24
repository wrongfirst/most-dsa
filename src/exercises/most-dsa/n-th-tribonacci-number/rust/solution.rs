impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut t = [0, 1, 1];

        if n < 3 {
            return t[n as usize];
        }

        for i in 3..=n as usize {
            t[i % 3] = t[0] + t[1] + t[2];
        }

        t[n as usize % 3]
    }
}

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut res = n as i32;

        let mut i = 1;
        while i < n {
            if ratings[i] == ratings[i - 1] {
                i += 1;
                continue;
            }

            let mut inc = 0;
            while i < n && ratings[i] > ratings[i - 1] {
                inc += 1;
                res += inc;
                i += 1;
            }

            let mut dec = 0;
            while i < n && ratings[i] < ratings[i - 1] {
                dec += 1;
                res += dec;
                i += 1;
            }

            res -= inc.min(dec);
        }

        res
    }
}

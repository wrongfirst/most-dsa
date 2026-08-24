impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();
        let mut suffix_sum = vec![0i64; n];
        suffix_sum[n - 1] = piles[n - 1] as i64;
        for i in (0..n - 1).rev() {
            suffix_sum[i] = piles[i] as i64 + suffix_sum[i + 1];
        }

        let mut dp = vec![vec![0i64; n + 1]; n + 1];

        for i in (0..n).rev() {
            for m in 1..=n {
                for x in 1..=2 * m {
                    if i + x > n {
                        break;
                    }
                    let next_m = m.max(x);
                    dp[i][m] = dp[i][m].max(suffix_sum[i] - dp[i + x][next_m]);
                }
            }
        }

        dp[0][1] as i32
    }
}

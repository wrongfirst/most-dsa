impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let n = stone_value.len();
        let mut dp = [0i32; 4];

        for i in (0..n).rev() {
            let mut total = 0;
            dp[i % 4] = i32::MIN;
            for j in i..std::cmp::min(i + 3, n) {
                total += stone_value[j];
                dp[i % 4] = dp[i % 4].max(total - dp[(j + 1) % 4]);
            }
        }

        if dp[0] == 0 {
            "Tie".to_string()
        } else if dp[0] > 0 {
            "Alice".to_string()
        } else {
            "Bob".to_string()
        }
    }
}

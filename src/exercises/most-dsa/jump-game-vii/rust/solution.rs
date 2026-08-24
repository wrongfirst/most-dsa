impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();

        if chars[n - 1] == '1' {
            return false;
        }

        let mut dp = vec![false; n];
        dp[0] = true;
        let mut j = 0usize;

        for i in 0..n {
            if !dp[i] {
                continue;
            }

            j = j.max(i + min_jump as usize);
            while j < n.min(i + max_jump as usize + 1) {
                if chars[j] == '0' {
                    dp[j] = true;
                }
                j += 1;
            }
        }

        dp[n - 1]
    }
}

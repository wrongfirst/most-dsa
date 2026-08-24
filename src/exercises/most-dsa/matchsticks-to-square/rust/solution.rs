impl Solution {
    pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
        let total_length: i32 = matchsticks.iter().sum();
        if total_length % 4 != 0 {
            return false;
        }

        let length = total_length / 4;
        let mut sides = [0i32; 4];
        matchsticks.sort_by(|a, b| b.cmp(a)); // Sort in descending order

        fn dfs(matchsticks: &[i32], sides: &mut [i32; 4], i: usize, length: i32) -> bool {
            if i == matchsticks.len() {
                return true;
            }

            for side in 0..4 {
                if sides[side] + matchsticks[i] <= length {
                    sides[side] += matchsticks[i];
                    if dfs(matchsticks, sides, i + 1, length) {
                        return true;
                    }
                    sides[side] -= matchsticks[i];
                }

                if sides[side] == 0 {
                    break;
                }
            }

            false
        }

        dfs(&matchsticks, &mut sides, 0, length)
    }
}

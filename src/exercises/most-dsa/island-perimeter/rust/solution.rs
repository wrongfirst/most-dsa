impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut res = 0;

        for r in 0..m {
            for c in 0..n {
                if grid[r][c] == 1 {
                    res += 4;
                    if r > 0 && grid[r - 1][c] == 1 {
                        res -= 2;
                    }
                    if c > 0 && grid[r][c - 1] == 1 {
                        res -= 2;
                    }
                }
            }
        }

        res
    }
}

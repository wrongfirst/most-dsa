impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = obstacle_grid;
        let m = grid.len();
        let n = grid[0].len();

        if grid[0][0] == 1 || grid[m - 1][n - 1] == 1 {
            return 0;
        }

        grid[m - 1][n - 1] = 1;

        for r in (0..m).rev() {
            for c in (0..n).rev() {
                if r == m - 1 && c == n - 1 {
                    continue;
                }

                if grid[r][c] == 1 {
                    grid[r][c] = 0;
                } else {
                    let down = if r + 1 < m { grid[r + 1][c] } else { 0 };
                    let right = if c + 1 < n { grid[r][c + 1] } else { 0 };
                    grid[r][c] = down + right;
                }
            }
        }

        grid[0][0]
    }
}

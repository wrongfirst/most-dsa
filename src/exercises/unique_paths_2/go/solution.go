func uniquePathsWithObstacles(grid [][]int) int {
	m, n := len(grid), len(grid[0])
	if grid[0][0] == 1 || grid[m-1][n-1] == 1 {
		return 0
	}

	grid[m-1][n-1] = 1

	for r := m - 1; r >= 0; r-- {
		for c := n - 1; c >= 0; c-- {
			if r == m-1 && c == n-1 {
				continue
			}
			if grid[r][c] == 1 {
				grid[r][c] = 0
			} else {
				down := 0
				if r+1 < m {
					down = grid[r+1][c]
				}
				right := 0
				if c+1 < n {
					right = grid[r][c+1]
				}
				grid[r][c] = down + right
			}
		}
	}

	return grid[0][0]
}

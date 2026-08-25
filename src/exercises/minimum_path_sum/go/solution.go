func minPathSum(grid [][]int) int {
	rows, cols := len(grid), len(grid[0])
	dp := make([]int, cols+1)
	for i := range dp {
		dp[i] = math.MaxInt32
	}
	dp[cols-1] = 0
	for r := rows - 1; r >= 0; r-- {
		for c := cols - 1; c >= 0; c-- {
			dp[c] = grid[r][c] + min(dp[c], dp[c+1])
		}
	}
	return dp[0]
}


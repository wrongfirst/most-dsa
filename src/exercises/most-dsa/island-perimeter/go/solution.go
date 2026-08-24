func islandPerimeter(grid [][]int) int {
	m, n := len(grid), len(grid[0])
	res := 0

	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			if grid[r][c] == 1 {
				res += 4

				if r > 0 && grid[r-1][c] == 1 {
					res -= 2
				}

				if c > 0 && grid[r][c-1] == 1 {
					res -= 2
				}
			}
		}
	}

	return res
}

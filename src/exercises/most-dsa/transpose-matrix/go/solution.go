func transpose(matrix [][]int) [][]int {
	rows := len(matrix)
	cols := len(matrix[0])
	if rows == cols {
		for r := 0; r < rows; r++ {
			for c := 0; c < r; c++ {
				matrix[r][c], matrix[c][r] = matrix[c][r], matrix[r][c]
			}
		}

		return matrix
	}
	res := make([][]int, cols)
	for i := 0; i < cols; i++ {
		res[i] = make([]int, rows)
	}
	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			res[c][r] = matrix[r][c]
		}
	}
	return res
}

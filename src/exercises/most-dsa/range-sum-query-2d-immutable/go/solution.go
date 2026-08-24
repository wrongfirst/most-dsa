type NumMatrix struct {
	sumMat [][]int
}

func Constructor(matrix [][]int) NumMatrix {
	rows, cols := len(matrix), len(matrix[0])
	sumMat := make([][]int, rows+1)
	for i := range sumMat {
		sumMat[i] = make([]int, cols+1)
	}

	for r := 0; r < rows; r++ {
		prefix := 0
		for c := 0; c < cols; c++ {
			prefix += matrix[r][c]
			above := sumMat[r][c+1]
			sumMat[r+1][c+1] = prefix + above
		}
	}

	return NumMatrix{sumMat: sumMat}
}

func (this *NumMatrix) SumRegion(row1 int, col1 int, row2 int, col2 int) int {
	row1, col1, row2, col2 = row1+1, col1+1, row2+1, col2+1

	bottomRight := this.sumMat[row2][col2]
	above := this.sumMat[row1-1][col2]
	left := this.sumMat[row2][col1-1]
	topLeft := this.sumMat[row1-1][col1-1]

	return bottomRight - above - left + topLeft
}

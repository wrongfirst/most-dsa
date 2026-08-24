func totalNQueens(n int) int {
	col := 0
	posDiag := 0
	negDiag := 0
	res := 0

	var backtrack func(r int)
	backtrack = func(r int) {
		if r == n {
			res++
			return
		}

		for c := 0; c < n; c++ {
			if ((col&(1<<c)) != 0) || ((posDiag&(1<<(r+c))) != 0) || ((negDiag&(1<<(r-c+n))) != 0) {
				continue
			}

			col ^= (1 << c)
			posDiag ^= (1 << (r + c))
			negDiag ^= (1 << (r - c + n))

			backtrack(r + 1)

			col ^= (1 << c)
			posDiag ^= (1 << (r + c))
			negDiag ^= (1 << (r - c + n))
		}
	}

	backtrack(0)
	return res
}

func numSquares(n int) int {
	for n%4 == 0 {
		n /= 4
	}

	if n%8 == 7 {
		return 4
	}

	isSquareNum := func(num int) bool {
		s := int(math.Sqrt(float64(num)))
		return s*s == num
	}

	if isSquareNum(n) {
		return 1
	}

	i := 1
	for i*i <= n {
		if isSquareNum(n - i*i) {
			return 2
		}
		i++
	}

	return 3
}

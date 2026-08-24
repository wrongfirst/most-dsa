func integerBreak(n int) int {
	if n <= 3 {
		return n - 1
	}
	res := int(math.Pow(3, float64(n/3)))
	if n%3 == 1 {
		return (res / 3) * 4
	}
	return res * max(1, n%3)
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

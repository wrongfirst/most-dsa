func minEnd(n int, x int) int {
	res := x
	iX := 1
	iN := 1
	for iN <= n-1 {
		if iX&x == 0 {
			if iN&(n-1) != 0 {
				res = res | iX
			}
			iN = iN << 1
		}
		iX = iX << 1
	}
	return res
}

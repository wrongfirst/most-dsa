func convertToTitle(columnNumber int) string {
	var res []byte
	for columnNumber > 0 {
		columnNumber--
		offset := columnNumber % 26
		res = append(res, byte('A'+offset))
		columnNumber /= 26
	}

	for i, j := 0, len(res)-1; i < j; i, j = i+1, j-1 {
		res[i], res[j] = res[j], res[i]
	}
	return string(res)
}

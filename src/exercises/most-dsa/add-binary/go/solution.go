func addBinary(a string, b string) string {
	var res []int
	carry := 0
	i := len(a) - 1
	j := len(b) - 1
	for i >= 0 || j >= 0 || carry > 0 {
		digitA := 0
		if i >= 0 {
			digitA = int(a[i] - '0')
		}
		digitB := 0
		if j >= 0 {
			digitB = int(b[j] - '0')
		}
		total := digitA + digitB + carry
		res = append(res, total%2)
		carry = total / 2
		i--
		j--
	}
	for left, right := 0, len(res)-1; left < right; left, right = left+1, right-1 {
		res[left], res[right] = res[right], res[left]
	}
	var builder strings.Builder
	for _, digit := range res {
		builder.WriteString(strconv.Itoa(digit))
	}
	return builder.String()
}

func longestDiverseString(a int, b int, c int) string {
	count := []int{a, b, c}
	res := []byte{}

	getMax := func(repeated int) int {
		idx := -1
		maxCnt := 0

		for i := 0; i < 3; i++ {
			if i == repeated || count[i] == 0 {
				continue
			}
			if maxCnt < count[i] {
				maxCnt = count[i]
				idx = i
			}
		}

		return idx
	}

	repeated := -1
	for {
		maxChar := getMax(repeated)
		if maxChar == -1 {
			break
		}

		res = append(res, byte(maxChar+'a'))
		count[maxChar]--

		if len(res) > 1 && res[len(res)-1] == res[len(res)-2] {
			repeated = maxChar
		} else {
			repeated = -1
		}
	}

	return string(res)
}

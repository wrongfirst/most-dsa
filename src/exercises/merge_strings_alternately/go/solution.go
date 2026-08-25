func mergeAlternately(word1 string, word2 string) string {
	n, m := len(word1), len(word2)
	res := make([]byte, 0, n+m)

	for i := 0; i < max(m, n); i++ {
		if i < n {
			res = append(res, word1[i])
		}
		if i < m {
			res = append(res, word2[i])
		}
	}
	return string(res)
}

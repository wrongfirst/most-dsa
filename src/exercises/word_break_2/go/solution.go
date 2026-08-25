func wordBreak(s string, wordDict []string) []string {
	wordSet := make(map[string]bool)
	for _, word := range wordDict {
		wordSet[word] = true
	}

	n := len(s)
	dp := make([][]string, n+1)
	for i := range dp {
		dp[i] = []string{}
	}
	dp[0] = []string{""}

	for i := 1; i <= n; i++ {
		for j := 0; j < i; j++ {
			if wordSet[s[j:i]] {
				for _, sentence := range dp[j] {
					newSentence := strings.TrimSpace(sentence + " " + s[j:i])
					dp[i] = append(dp[i], newSentence)
				}
			}
		}
	}

	return dp[n]
}

func stoneGameIII(stoneValue []int) string {
	n := len(stoneValue)
	dp := make([]int, 4)
	for i := n - 1; i >= 0; i-- {
		total := 0
		dp[i%4] = math.MinInt32
		for j := i; j < min(i+3, n); j++ {
			total += stoneValue[j]
			dp[i%4] = max(dp[i%4], total-dp[(j+1)%4])
		}
	}
	if dp[0] == 0 {
		return "Tie"
	}
	if dp[0] > 0 {
		return "Alice"
	}
	return "Bob"
}


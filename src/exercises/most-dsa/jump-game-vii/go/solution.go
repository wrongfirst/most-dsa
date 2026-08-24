func canReach(s string, minJump int, maxJump int) bool {
	n := len(s)
	if s[n-1] == '1' {
		return false
	}

	dp := make([]bool, n)
	dp[0] = true
	j := 0
	for i := 0; i < n; i++ {
		if dp[i] == false {
			continue
		}
		if i+minJump > j {
			j = i + minJump
		}
		limit := i + maxJump + 1
		if n < limit {
			limit = n
		}
		for j < limit {
			if s[j] == '0' {
				dp[j] = true
			}
			j++
		}
	}

	return dp[n-1]
}

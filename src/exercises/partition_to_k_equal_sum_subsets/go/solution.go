func canPartitionKSubsets(nums []int, k int) bool {
	total := 0
	for _, num := range nums {
		total += num
	}

	if total%k != 0 {
		return false
	}

	target := total / k
	n := len(nums)
	N := 1 << n

	dp := make([]int, N)
	for i := 1; i < N; i++ {
		dp[i] = -1
	}
	dp[0] = 0

	for mask := 0; mask < N; mask++ {
		if dp[mask] == -1 {
			continue
		}

		for i := 0; i < n; i++ {
			if (mask&(1<<i)) == 0 && dp[mask]+nums[i] <= target {
				dp[mask|(1<<i)] = (dp[mask] + nums[i]) % target
			}
		}
	}

	return dp[N-1] == 0
}

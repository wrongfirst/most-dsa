func lastStoneWeightII(stones []int) int {
	stoneSum := 0
	for _, stone := range stones {
		stoneSum += stone
	}
	target := stoneSum / 2
	dp := make(map[int]bool)
	dp[0] = true
	for _, stone := range stones {
		newDp := make(map[int]bool)
		for val := range dp {
			newDp[val] = true
		}
		for val := range dp {
			if val+stone == target {
				return stoneSum - 2*target
			}
			if val+stone < target {
				newDp[val+stone] = true
			}
		}
		dp = newDp
	}
	maxVal := 0
	for val := range dp {
		if val > maxVal {
			maxVal = val
		}
	}
	return stoneSum - 2*maxVal
}

func combinationSum4(nums []int, target int) int {
	sort.Ints(nums)
	dp := make(map[int]int)
	dp[target] = 1
	for total := target; total > 0; total-- {
		for _, num := range nums {
			if total < num {
				break
			}
			dp[total-num] += dp[total]
		}
	}
	return dp[0]
}

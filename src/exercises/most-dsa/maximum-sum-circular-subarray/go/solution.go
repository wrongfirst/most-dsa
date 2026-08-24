func maxSubarraySumCircular(nums []int) int {
	globMax, globMin := nums[0], nums[0]
	curMax, curMin := 0, 0
	total := 0
	for _, num := range nums {
		curMax = max(curMax+num, num)
		curMin = min(curMin+num, num)
		total += num
		globMax = max(globMax, curMax)
		globMin = min(globMin, curMin)
	}
	if globMax > 0 {
		return max(globMax, total-globMin)
	}
	return globMax
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

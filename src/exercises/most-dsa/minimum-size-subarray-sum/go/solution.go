func minSubArrayLen(target int, nums []int) int {
	l, total := 0, 0
	res := int(1e9)

	for r := 0; r < len(nums); r++ {
		total += nums[r]

		for total >= target {
			res = min(r-l+1, res)
			total -= nums[l]
			l++
		}
	}

	if res == int(1e9) {
		return 0
	}
	return res
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

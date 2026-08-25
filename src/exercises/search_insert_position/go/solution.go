func searchInsert(nums []int, target int) int {
	l, r := 0, len(nums)

	for l < r {
		m := l + ((r - l) / 2)

		if nums[m] >= target {
			r = m
		} else if nums[m] < target {
			l = m + 1
		}
	}

	return l
}

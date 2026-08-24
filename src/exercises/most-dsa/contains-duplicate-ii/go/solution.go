func containsNearbyDuplicate(nums []int, k int) bool {
	window := make(map[int]bool)
	l := 0

	for r := 0; r < len(nums); r++ {
		if r-l > k {
			delete(window, nums[l])
			l++
		}
		if window[nums[r]] {
			return true
		}
		window[nums[r]] = true
	}

	return false
}

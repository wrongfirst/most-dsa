func search(nums []int, target int) bool {
	for _, num := range nums {
		if num == target {
			return true
		}
	}
	return false
}

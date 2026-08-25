func firstMissingPositive(nums []int) int {
	for i := range nums {
		if nums[i] < 0 {
			nums[i] = 0
		}
	}

	for i := range nums {
		val := abs(nums[i])
		if 1 <= val && val <= len(nums) {
			if nums[val-1] > 0 {
				nums[val-1] *= -1
			} else if nums[val-1] == 0 {
				nums[val-1] = -1 * (len(nums) + 1)
			}
		}
	}

	for i := 1; i <= len(nums); i++ {
		if nums[i-1] >= 0 {
			return i
		}
	}

	return len(nums) + 1
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

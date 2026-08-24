func permuteUnique(nums []int) [][]int {
	res := [][]int{}

	var dfs func(i int)
	dfs = func(i int) {
		if i == len(nums) {
			numsCopy := make([]int, len(nums))
			copy(numsCopy, nums)
			res = append(res, numsCopy)
			return
		}

		for j := i; j < len(nums); j++ {
			if j > i && nums[i] == nums[j] {
				continue
			}
			nums[i], nums[j] = nums[j], nums[i]
			dfs(i + 1)
		}

		for j := len(nums) - 1; j > i; j-- {
			nums[j], nums[i] = nums[i], nums[j]
		}
	}

	sort.Ints(nums)
	dfs(0)
	return res
}

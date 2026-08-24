func majorityElement(nums []int) []int {
	count := make(map[int]int)

	for _, num := range nums {
		count[num]++

		if len(count) <= 2 {
			continue
		}

		newCount := make(map[int]int)
		for num, c := range count {
			if c > 1 {
				newCount[num] = c - 1
			}
		}
		count = newCount
	}

	res := []int{}
	for num := range count {
		actualCount := 0
		for _, n := range nums {
			if n == num {
				actualCount++
			}
		}
		if actualCount > len(nums)/3 {
			res = append(res, num)
		}
	}

	return res
}

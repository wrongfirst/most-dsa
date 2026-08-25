func shipWithinDays(weights []int, days int) int {
	l, r := max(weights...), sum(weights)
	res := r

	canShip := func(cap int) bool {
		ships, currCap := 1, cap

		for _, w := range weights {
			if currCap-w < 0 {
				ships++
				if ships > days {
					return false
				}
				currCap = cap
			}
			currCap -= w
		}

		return true
	}

	for l <= r {
		cap := (l + r) / 2

		if canShip(cap) {
			res = min(res, cap)
			r = cap - 1
		} else {
			l = cap + 1
		}
	}

	return res
}

func max(nums ...int) int {
	maxVal := nums[0]
	for _, num := range nums {
		if num > maxVal {
			maxVal = num
		}
	}
	return maxVal
}

func sum(nums []int) int {
	total := 0
	for _, num := range nums {
		total += num
	}
	return total
}

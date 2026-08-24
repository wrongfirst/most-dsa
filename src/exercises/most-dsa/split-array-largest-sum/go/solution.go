func splitArray(nums []int, k int) int {
	n := len(nums)
	prefix := make([]int, n+1)

	for i := 0; i < n; i++ {
		prefix[i+1] = prefix[i] + nums[i]
	}

	canSplit := func(largest int) bool {
		subarrays := 0
		i := 0

		for i < n {
			l, r := i+1, n

			for l <= r {
				mid := l + (r-l)/2

				if prefix[mid]-prefix[i] <= largest {
					l = mid + 1
				} else {
					r = mid - 1
				}
			}

			subarrays++
			i = r

			if subarrays > k {
				return false
			}
		}

		return true
	}

	l, r := max(nums...), sum(nums)
	res := r

	for l <= r {
		mid := l + (r-l)/2

		if canSplit(mid) {
			res = mid
			r = mid - 1
		} else {
			l = mid + 1
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

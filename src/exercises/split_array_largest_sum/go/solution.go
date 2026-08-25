func splitArray(nums []int, k int) int {
	n := len(nums)
	prefix := make([]int, n+1)
	maxNum, sumNum := nums[0], 0

	for i := 0; i < n; i++ {
		prefix[i+1] = prefix[i] + nums[i]
		if nums[i] > maxNum {
			maxNum = nums[i]
		}
		sumNum += nums[i]
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

	l, r := maxNum, sumNum
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


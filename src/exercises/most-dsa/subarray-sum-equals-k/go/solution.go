func subarraySum(nums []int, k int) int {
	res := 0
	curSum := 0
	prefixSums := map[int]int{0: 1}

	for _, num := range nums {
		curSum += num
		diff := curSum - k

		res += prefixSums[diff]

		prefixSums[curSum] = 1 + prefixSums[curSum]
	}

	return res
}

func candy(ratings []int) int {
	n := len(ratings)
	res := n
	i := 1
	for i < n {
		if ratings[i] == ratings[i-1] {
			i++
			continue
		}

		inc := 0
		for i < n && ratings[i] > ratings[i-1] {
			inc++
			res += inc
			i++
		}
		dec := 0
		for i < n && ratings[i] < ratings[i-1] {
			dec++
			res += dec
			i++
		}

		if inc < dec {
			res -= inc
		} else {
			res -= dec
		}
	}

	return res
}

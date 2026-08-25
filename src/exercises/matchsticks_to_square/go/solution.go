func makesquare(matchsticks []int) bool {
	totalLength := 0
	for _, stick := range matchsticks {
		totalLength += stick
	}

	if totalLength%4 != 0 {
		return false
	}

	length := totalLength / 4
	sides := [4]int{}

	sort.Slice(matchsticks, func(i, j int) bool {
		return matchsticks[i] > matchsticks[j]
	})

	var dfs func(i int) bool
	dfs = func(i int) bool {
		if i == len(matchsticks) {
			return true
		}

		for side := 0; side < 4; side++ {
			if sides[side]+matchsticks[i] <= length {
				sides[side] += matchsticks[i]
				if dfs(i + 1) {
					return true
				}
				sides[side] -= matchsticks[i]

				if sides[side] == 0 {
					break
				}
			}
		}

		return false
	}

	return dfs(0)
}

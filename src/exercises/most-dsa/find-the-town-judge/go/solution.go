func findJudge(n int, trust [][]int) int {
	delta := make(map[int]int)

	for _, edge := range trust {
		src, dst := edge[0], edge[1]
		delta[src]--
		delta[dst]++
	}

	for i := 1; i <= n; i++ {
		if delta[i] == n-1 {
			return i
		}
	}

	return -1
}

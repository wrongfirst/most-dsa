func findMinHeightTrees(n int, edges [][]int) []int {
	if n == 1 {
		return []int{0}
	}

	adj := make(map[int][]int)
	for _, edge := range edges {
		n1, n2 := edge[0], edge[1]
		adj[n1] = append(adj[n1], n2)
		adj[n2] = append(adj[n2], n1)
	}

	edgeCnt := make(map[int]int)
	leaves := []int{}

	for src, neighbors := range adj {
		edgeCnt[src] = len(neighbors)
		if len(neighbors) == 1 {
			leaves = append(leaves, src)
		}
	}

	for len(leaves) > 0 {
		if n <= 2 {
			return leaves
		}

		leavesLen := len(leaves)
		for i := 0; i < leavesLen; i++ {
			node := leaves[0]
			leaves = leaves[1:]
			n--

			for _, nei := range adj[node] {
				edgeCnt[nei]--
				if edgeCnt[nei] == 1 {
					leaves = append(leaves, nei)
				}
			}
		}
	}

	return []int{}
}

func buildMatrix(k int, rowConditions [][]int, colConditions [][]int) [][]int {
	topoSort := func(edges [][]int) []int {
		indegree := make([]int, k+1)
		adj := make([][]int, k+1)
		for i := range adj {
			adj[i] = []int{}
		}
		for _, edge := range edges {
			u, v := edge[0], edge[1]
			adj[u] = append(adj[u], v)
			indegree[v]++
		}

		order := []int{}
		q := []int{}
		for i := 1; i <= k; i++ {
			if indegree[i] == 0 {
				q = append(q, i)
			}
		}

		for len(q) > 0 {
			node := q[0]
			q = q[1:]
			order = append(order, node)
			for _, nei := range adj[node] {
				indegree[nei]--
				if indegree[nei] == 0 {
					q = append(q, nei)
				}
			}
		}

		return order
	}
	rowOrder := topoSort(rowConditions)
	if len(rowOrder) != k {
		return [][]int{}
	}

	colOrder := topoSort(colConditions)
	if len(colOrder) != k {
		return [][]int{}
	}

	res := make([][]int, k)
	for i := range res {
		res[i] = make([]int, k)
	}
	colIndex := make([]int, k+1)
	for i := 0; i < k; i++ {
		colIndex[colOrder[i]] = i
	}

	for i := 0; i < k; i++ {
		res[i][colIndex[rowOrder[i]]] = rowOrder[i]
	}
	return res
}

type Edge struct {
	node   string
	weight float64
}

func calcEquation(equations [][]string, values []float64, queries [][]string) []float64 {
	adj := make(map[string][]Edge)

	for i, eq := range equations {
		a, b := eq[0], eq[1]
		adj[a] = append(adj[a], Edge{node: b, weight: values[i]})
		adj[b] = append(adj[b], Edge{node: a, weight: 1.0 / values[i]})
	}

	bfs := func(src, target string) float64 {
		if _, existsSrc := adj[src]; !existsSrc {
			return -1.0
		}
		if _, existsTarget := adj[target]; !existsTarget {
			return -1.0
		}

		type QueueItem struct {
			node   string
			weight float64
		}

		q := []QueueItem{{node: src, weight: 1.0}}
		visit := make(map[string]bool)
		visit[src] = true

		for len(q) > 0 {
			item := q[0]
			q = q[1:]

			if item.node == target {
				return item.weight
			}

			for _, edge := range adj[item.node] {
				if !visit[edge.node] {
					q = append(q, QueueItem{node: edge.node, weight: item.weight * edge.weight})
					visit[edge.node] = true
				}
			}
		}

		return -1.0
	}

	result := make([]float64, len(queries))
	for i, query := range queries {
		result[i] = bfs(query[0], query[1])
	}

	return result
}

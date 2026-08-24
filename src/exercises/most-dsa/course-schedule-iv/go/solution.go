func checkIfPrerequisite(numCourses int, prerequisites [][]int, queries [][]int) []bool {
	adj := make([]map[int]bool, numCourses)
	for i := range adj {
		adj[i] = make(map[int]bool)
	}

	indegree := make([]int, numCourses)
	isPrereq := make([]map[int]bool, numCourses)
	for i := range isPrereq {
		isPrereq[i] = make(map[int]bool)
	}

	for _, edge := range prerequisites {
		pre, crs := edge[0], edge[1]
		adj[pre][crs] = true
		indegree[crs]++
	}

	q := []int{}
	for i := 0; i < numCourses; i++ {
		if indegree[i] == 0 {
			q = append(q, i)
		}
	}

	for len(q) > 0 {
		node := q[0]
		q = q[1:]

		for neighbor := range adj[node] {
			isPrereq[neighbor][node] = true
			for prereqNode := range isPrereq[node] {
				isPrereq[neighbor][prereqNode] = true
			}

			indegree[neighbor]--
			if indegree[neighbor] == 0 {
				q = append(q, neighbor)
			}
		}
	}

	result := make([]bool, len(queries))
	for i, query := range queries {
		u, v := query[0], query[1]
		result[i] = isPrereq[v][u]
	}

	return result
}

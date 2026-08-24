func canTraverseAllPairs(nums []int) bool {
	n := len(nums)
	if n == 1 {
		return true
	}
	for _, num := range nums {
		if num == 1 {
			return false
		}
	}
	maxNum := nums[0]
	for _, num := range nums {
		if num > maxNum {
			maxNum = num
		}
	}
	sieve := make([]int, maxNum+1)
	p := 2
	for p*p <= maxNum {
		if sieve[p] == 0 {
			for composite := p * p; composite <= maxNum; composite += p {
				sieve[composite] = p
			}
		}
		p++
	}
	adj := make(map[int][]int)
	for i := 0; i < n; i++ {
		num := nums[i]
		if sieve[num] == 0 {
			adj[i] = append(adj[i], n+num)
			adj[n+num] = append(adj[n+num], i)
			continue
		}
		for num > 1 {
			prime := sieve[num]
			if prime == 0 {
				prime = num
			}
			adj[i] = append(adj[i], n+prime)
			adj[n+prime] = append(adj[n+prime], i)
			for num%prime == 0 {
				num /= prime
			}
		}
	}
	visited := make(map[int]bool)
	queue := []int{0}
	visited[0] = true
	for len(queue) > 0 {
		node := queue[0]
		queue = queue[1:]
		for _, nei := range adj[node] {
			if !visited[nei] {
				visited[nei] = true
				queue = append(queue, nei)
			}
		}
	}
	for i := 0; i < n; i++ {
		if !visited[i] {
			return false
		}
	}
	return true
}

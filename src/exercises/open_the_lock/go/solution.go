func openLock(deadends []string, target string) int {
	if target == "0000" {
		return 0
	}

	visit := make(map[string]bool)
	for _, deadend := range deadends {
		visit[deadend] = true
	}

	if visit["0000"] {
		return -1
	}

	q := []string{"0000"}
	visit["0000"] = true
	steps := 0

	for len(q) > 0 {
		steps++
		qLen := len(q)

		for k := 0; k < qLen; k++ {
			lock := q[0]
			q = q[1:]

			for i := 0; i < 4; i++ {
				for _, j := range []int{1, -1} {
					digit := (int(lock[i]-'0') + j + 10) % 10
					nextLock := lock[:i] + strconv.Itoa(digit) + lock[i+1:]

					if visit[nextLock] {
						continue
					}

					if nextLock == target {
						return steps
					}

					q = append(q, nextLock)
					visit[nextLock] = true
				}
			}
		}
	}

	return -1
}

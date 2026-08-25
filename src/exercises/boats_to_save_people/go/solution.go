func numRescueBoats(people []int, limit int) int {
	sort.Ints(people)
	res := 0
	l, r := 0, len(people)-1

	for l <= r {
		remain := limit - people[r]
		r--
		res++
		if l <= r && remain >= people[l] {
			l++
		}
	}

	return res
}

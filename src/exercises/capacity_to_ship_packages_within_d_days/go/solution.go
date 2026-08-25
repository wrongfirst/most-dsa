func shipWithinDays(weights []int, days int) int {
	maxW, totalW := 0, 0
	for _, w := range weights {
		if w > maxW {
			maxW = w
		}
		totalW += w
	}
	l, r := maxW, totalW
	res := r

	canShip := func(cap int) bool {
		ships, currCap := 1, cap

		for _, w := range weights {
			if currCap-w < 0 {
				ships++
				if ships > days {
					return false
				}
				currCap = cap
			}
			currCap -= w
		}

		return true
	}

	for l <= r {
		cap := (l + r) / 2

		if canShip(cap) {
			res = min(res, cap)
			r = cap - 1
		} else {
			l = cap + 1
		}
	}

	return res
}


func predictPartyVictory(senate string) string {
	senateSlice := []rune(senate)
	cnt := 0
	i := 0
	for i < len(senateSlice) {
		c := senateSlice[i]
		if c == 'R' {
			if cnt < 0 {
				senateSlice = append(senateSlice, 'D')
			}
			cnt++
		} else {
			if cnt > 0 {
				senateSlice = append(senateSlice, 'R')
			}
			cnt--
		}
		i++
	}

	if cnt > 0 {
		return "Radiant"
	}
	return "Dire"
}

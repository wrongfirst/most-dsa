func reorganizeString(s string) string {
	freq := make([]int, 26)
	for _, char := range s {
		freq[char-'a']++
	}

	maxIdx := 0
	for i := 0; i < 26; i++ {
		if freq[i] > freq[maxIdx] {
			maxIdx = i
		}
	}

	maxFreq := freq[maxIdx]
	if maxFreq > (len(s)+1)/2 {
		return ""
	}

	res := make([]byte, len(s))
	idx := 0
	maxChar := byte(maxIdx + 'a')

	for freq[maxIdx] > 0 {
		res[idx] = maxChar
		idx += 2
		freq[maxIdx]--
	}

	for i := 0; i < 26; i++ {
		for freq[i] > 0 {
			if idx >= len(s) {
				idx = 1
			}
			res[idx] = byte(i + 'a')
			idx += 2
			freq[i]--
		}
	}

	return string(res)
}

func gcdOfStrings(str1 string, str2 string) string {
	g := gcd(len(str1), len(str2))

	valid1 := true
	for i := 0; i < len(str1); i++ {
		if str1[i] != str1[i%g] {
			valid1 = false
			break
		}
	}

	valid2 := true
	for i := 0; i < len(str2); i++ {
		if str2[i] != str1[i%g] {
			valid2 = false
			break
		}
	}

	if valid1 && valid2 {
		return str1[:g]
	}
	return ""
}

func gcd(a int, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

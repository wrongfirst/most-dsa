func maxTurbulenceSize(arr []int) int {
	n := len(arr)
	res := 0
	cnt := 0
	sign := -1

	for i := 0; i < n-1; i++ {
		if arr[i] > arr[i+1] {
			if sign == 0 {
				cnt = cnt + 1
			} else {
				cnt = 1
			}
			sign = 1
		} else if arr[i] < arr[i+1] {
			if sign == 1 {
				cnt = cnt + 1
			} else {
				cnt = 1
			}
			sign = 0
		} else {
			cnt = 0
			sign = -1
		}

		if cnt > res {
			res = cnt
		}
	}

	return res + 1
}

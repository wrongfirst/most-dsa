/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * func guess(num int) int;
 */

func guess(num int) int {
	return 0
}

func guessNumber(n int) int {
	l, r := 1, n

	for {
		m := (l + r) / 2
		res := guess(m)

		if res > 0 {
			l = m + 1
		} else if res < 0 {
			r = m - 1
		} else {
			return m
		}
	}
}

/**
 * // This is the MountainArray's API interface.
 * // You should not implement it, or speculate about its implementation
 * type MountainArray struct {
 * }
 *
 * func (this *MountainArray) get(index int) int {}
 * func (this *MountainArray) length() int {}
 */

 func findInMountainArray(target int, mountainArr *MountainArray) int {
    length := mountainArr.length()

    l, r := 0, length-1
    for l < r {
        m := (l + r) / 2
        if mountainArr.get(m) < mountainArr.get(m+1) {
            l = m + 1
        } else {
            r = m
        }
    }
    peak := l

    l, r = 0, peak
    for l <= r {
        m := (l + r) / 2
        val := mountainArr.get(m)
        if val < target {
            l = m + 1
        } else if val > target {
            r = m - 1
        } else {
            return m
        }
    }

    l, r = peak+1, length-1
    for l <= r {
        m := (l + r) / 2
        val := mountainArr.get(m)
        if val < target {
            r = m - 1
        } else if val > target {
            l = m + 1
        } else {
            return m
        }
    }

    return -1
}

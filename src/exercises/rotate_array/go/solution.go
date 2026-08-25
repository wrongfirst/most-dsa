func rotate(nums []int, k int) {
    n := len(nums)
    k %= n

    reverse := func(l, r int) {
        for l < r {
            nums[l], nums[r] = nums[r], nums[l]
            l++
            r--
        }
    }

    reverse(0, n-1)
    reverse(0, k-1)
    reverse(k, n-1)
}

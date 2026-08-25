func removeElement(nums []int, val int) int {
    i := 0
    n := len(nums)

    for i < n {
        if nums[i] == val {
            n--
            nums[i] = nums[n]
        } else {
            i++
        }
    }
    return n
}

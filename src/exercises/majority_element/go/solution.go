func majorityElement(nums []int) int {
    res, count := 0, 0

    for _, num := range nums {
        if count == 0 {
            res = num
        }
        if num == res {
            count++
        } else {
            count--
        }
    }
    return res
}

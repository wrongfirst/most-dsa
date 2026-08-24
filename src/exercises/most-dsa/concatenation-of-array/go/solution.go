func getConcatenation(nums []int) []int {
    ans := []int{}

    for i := 0; i < 2; i++ {
        for _, num := range nums {
            ans = append(ans, num)
        }
    }
    return ans
}

func sortColors(nums []int) {
    zero, one := 0, 0

    for two := 0; two < len(nums); two++ {
        tmp := nums[two]
        nums[two] = 2
        if tmp < 2 {
            nums[one] = 1
            one++
        }
        if tmp < 1 {
            nums[zero] = 0
            zero++
        }
    }
}

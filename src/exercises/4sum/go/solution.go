func fourSum(nums []int, target int) [][]int {
    sort.Ints(nums)
    n := len(nums)
    res := [][]int{}

    for i := 0; i < n; i++ {
        if i > 0 && nums[i] == nums[i-1] {
            continue
        }
        for j := i + 1; j < n; j++ {
            if j > i+1 && nums[j] == nums[j-1] {
                continue
            }
            left, right := j+1, n-1
            for left < right {
                total := nums[i] + nums[j] + nums[left] + nums[right]
                if total == target {
                    res = append(res, []int{nums[i], nums[j], nums[left], nums[right]})
                    left++
                    right--
                    for left < right && nums[left] == nums[left-1] {
                        left++
                    }
                    for left < right && nums[right] == nums[right+1] {
                        right--
                    }
                } else if total < target {
                    left++
                } else {
                    right--
                }
            }
        }
    }

    return res
}

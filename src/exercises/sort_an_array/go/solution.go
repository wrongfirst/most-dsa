func sortArray(nums []int) []int {
    var merge func(arr []int, l, m, r int)
    merge = func(arr []int, l, m, r int) {
        left := make([]int, m-l+1)
        right := make([]int, r-m)
        copy(left, arr[l:m+1])
        copy(right, arr[m+1:r+1])

        i, j, k := l, 0, 0
        for j < len(left) && k < len(right) {
            if left[j] <= right[k] {
                arr[i] = left[j]
                j++
            } else {
                arr[i] = right[k]
                k++
            }
            i++
        }

        for j < len(left) {
            arr[i] = left[j]
            j++
            i++
        }

        for k < len(right) {
            arr[i] = right[k]
            k++
            i++
        }
    }

    var mergeSort func(arr []int, l, r int)
    mergeSort = func(arr []int, l, r int) {
        if l == r {
            return
        }
        m := (l + r) / 2
        mergeSort(arr, l, m)
        mergeSort(arr, m+1, r)
        merge(arr, l, m, r)
    }

    mergeSort(nums, 0, len(nums)-1)
    return nums
}

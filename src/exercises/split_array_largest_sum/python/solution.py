def splitArray(nums: list[int], k: int) -> int:
    n = len(nums)
    prefix = [0] * (n + 1)
    for i in range(n):
        prefix[i + 1] = prefix[i] + nums[i]

    def canSplit(largest):
        subarrays = 0
        i = 0
        while i < n:
            l, r = i + 1, n
            while l <= r:
                mid = l + (r - l) // 2
                if prefix[mid] - prefix[i] <= largest:
                    l = mid + 1
                else:
                    r = mid - 1
            subarrays += 1
            i = r
            if subarrays > k:
                return False
        return True

    l, r = max(nums), sum(nums)
    res = r
    while l <= r:
        mid = l + (r - l) // 2
        if canSplit(mid):
            res = mid
            r = mid - 1
        else:
            l = mid + 1

    return res

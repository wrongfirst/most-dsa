def minSubArrayLen(target: int, nums: list[int]) -> int:
    l, total = 0, 0
    res = len(nums) + 1

    for r in range(len(nums)):
        total += nums[r]
        while total >= target:
            res = min(r - l + 1, res)
            total -= nums[l]
            l += 1

    return 0 if res > len(nums) else res

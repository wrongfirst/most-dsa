class Solution:
    def combinationSum4(self, nums: List[int], target: int) -> int:
        nums.sort()
        dp = defaultdict(int)
        dp[target] = 1
        for total in range(target, 0, -1):
            for num in nums:
                if total < num:
                    break
                dp[total - num] += dp[total]
        return dp[0]

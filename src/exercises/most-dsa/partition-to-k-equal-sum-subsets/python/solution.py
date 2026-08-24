class Solution:
    def canPartitionKSubsets(self, nums: List[int], k: int) -> bool:
        total = sum(nums)
        if total % k != 0:
            return False

        target = total // k
        n = len(nums)
        N = 1 << n
        dp = [0] + [-1] * (N - 1)

        for mask in range(N):
            if dp[mask] == -1:
                continue
            for i in range(n):
                if (mask & (1 << i)) == 0 and dp[mask] + nums[i] <= target:
                    dp[mask | (1 << i)] = (dp[mask] + nums[i]) % target

        return dp[N - 1] == 0

class Solution {
public:
    int combinationSum4(vector<int>& nums, int target) {
        sort(nums.begin(), nums.end());
        unordered_map<int, int> dp;
        dp[target] = 1;

        for (int total = target; total > 0; total--) {
            if (dp[total] == -1) continue;
            for (auto& num : nums) {
                if (total < num) break;
                if (dp[total - num] + 0LL + dp[total] > INT_MAX) {
                    dp[total - num] = -1;
                    break;
                }
                dp[total - num] += dp[total];
            }
        }
        return dp[0];
    }
};

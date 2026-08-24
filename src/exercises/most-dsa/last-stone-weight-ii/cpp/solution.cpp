class Solution {
public:
    int lastStoneWeightII(vector<int>& stones) {
        int stoneSum = accumulate(stones.begin(), stones.end(), 0);
        int target = stoneSum / 2;

        unordered_set<int> dp = {0};

        for (int& stone : stones) {
            unordered_set<int> newDp(dp);
            for (int val : dp) {
                if (val + stone == target) {
                    return stoneSum - 2 * target;
                }
                if (val + stone < target) {
                    newDp.insert(val + stone);
                }
            }
            dp = newDp;
        }

        int maxVal = 0;
        for (int val : dp) {
            maxVal = max(maxVal, val);
        }

        return stoneSum - 2 * maxVal;
    }
};

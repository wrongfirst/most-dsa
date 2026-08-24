class Solution {
private:
    vector<int> prefix;
    int n;

public:
    int splitArray(vector<int>& nums, int k) {
        n = nums.size();
        prefix.resize(n + 1, 0);
        for (int i = 0; i < n; ++i) {
            prefix[i + 1] = prefix[i] + nums[i];
        }

        int l = *max_element(nums.begin(), nums.end());
        int r = accumulate(nums.begin(), nums.end(), 0);
        int res = r;

        while (l <= r) {
            int mid = l + (r - l) / 2;
            if (canSplit(mid, k)) {
                res = mid;
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }

        return res;
    }

private:
    bool canSplit(int largest, int k) {
        int subarrays = 0, i = 0;
        while (i < n) {
            int l = i + 1, r = n;
            while (l <= r) {
                int mid = l + (r - l) / 2;
                if (prefix[mid] - prefix[i] <= largest) {
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }
            subarrays++;
            i = r;
            if (subarrays > k) {
                return false;
            }
        }
        return true;
    }
};

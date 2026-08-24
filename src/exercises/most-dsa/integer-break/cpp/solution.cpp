class Solution {
public:
    int integerBreak(int n) {
        if (n <= 3) {
            return n - 1;
        }

        int res = pow(3, n / 3);
        if (n % 3 == 1) {
            return (res / 3) * 4;
        }

        return res * max(1, n % 3);
    }
};

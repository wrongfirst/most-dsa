class Solution {
public:
    int numSquares(int n) {
        while (n % 4 == 0) {
            n /= 4;
        }
        
        if (n % 8 == 7) {
            return 4;
        }
        
        if (isSquareNum(n)) {
            return 1;
        }
        
        for (int i = 1; i * i <= n; i++) {
            if (isSquareNum(n - i * i)) {
                return 2;
            }
        }
        
        return 3;
    }
    
private:
    bool isSquareNum(int num) {
        int s = (int) sqrt(num);
        return s * s == num;
    }
};

class Solution {
public:
    string longestDiverseString(int a, int b, int c) {
        vector<int> count = {a, b, c};
        string res;

        int repeated = -1;
        while (true) {
            int maxChar = getMax(count, repeated);
            if (maxChar == -1) {
                break;
            }
            res += (char)(maxChar + 'a');
            count[maxChar]--;

            if (res.size() > 1 && res.back() == res[res.size() - 2]) {
                repeated = maxChar;
            } else {
                repeated = -1;
            }
        }

        return res;
    }

private:
    int getMax(const vector<int>& count, int repeated) {
        int idx = -1, maxCnt = 0;
        for (int i = 0; i < 3; i++) {
            if (i == repeated || count[i] == 0) {
                continue;
            }
            if (maxCnt < count[i]) {
                maxCnt = count[i];
                idx = i;
            }
        }
        return idx;
    }
};

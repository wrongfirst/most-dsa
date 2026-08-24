class Solution {
public:
    string reorganizeString(string s) {
        vector<int> freq(26, 0);
        for (char& c : s) {
            freq[c - 'a']++;
        }

        int maxIdx = max_element(freq.begin(), freq.end()) - freq.begin();
        int maxFreq = freq[maxIdx];
        if (maxFreq > (s.size() + 1) / 2) {
            return "";
        }

        string res(s.size(), ' ');
        int idx = 0;
        char maxChar = 'a' + maxIdx;

        while (freq[maxIdx] > 0) {
            res[idx] = maxChar;
            idx += 2;
            freq[maxIdx]--;
        }

        for (int i = 0; i < 26; i++) {
            while (freq[i] > 0) {
                if (idx >= s.size()) {
                    idx = 1;
                }
                res[idx] = 'a' + i;
                idx += 2;
                freq[i]--;
            }
        }

        return res;
    }
};

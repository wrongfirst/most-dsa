class Solution {
public:
    vector<string> wordBreak(string s, vector<string>& wordDict) {
        unordered_set<string> wordSet(wordDict.begin(), wordDict.end());
        int n = s.size();
        vector<vector<string>> dp(n + 1);
        dp[0] = {""};
        
        for (int i = 1; i <= n; ++i) {
            for (int j = 0; j < i; ++j) {
                string word = s.substr(j, i - j);
                if (wordSet.count(word)) {
                    for (const string& sentence : dp[j]) {
                        if (sentence.empty()) {
                            dp[i].push_back(word);
                        } else {
                            dp[i].push_back(sentence + " " + word);
                        }
                    }
                }
            }
        }
        
        return dp[n];
    }
};

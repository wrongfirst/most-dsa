class Solution {
public:
    string gcdOfStrings(string str1, string str2) {
        int g = __gcd((int)str1.size(), (int)str2.size());

        for (int i = 0; i < str1.size(); i++) {
            if (str1[i] != str1[i % g]) {
                return "";
            }
        }

        for (int i = 0; i < str2.size(); i++) {
            if (str2[i] != str1[i % g]) {
                return "";
            }
        }

        return str1.substr(0, g);
    }
};

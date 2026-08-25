class Solution:
    def gcdOfStrings(self, str1: str, str2: str) -> str:
        g = math.gcd(len(str1), len(str2))
        
        if all(str1[i] == str1[i % g] for i in range(len(str1))) and \
           all(str2[i] == str1[i % g] for i in range(len(str2))):
            return str1[:g]
        return ""

def getConcatenation(nums: List[int]) -> List[int]:
    ans = []
    for i in range(2):
        for num in nums:
            ans.append(num)
    return ans

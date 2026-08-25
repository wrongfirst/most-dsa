def reorganizeString(s: str) -> str:
    freq = [0] * 26
    for char in s:
        freq[ord(char) - ord('a')] += 1

    max_idx = freq.index(max(freq))
    max_freq = freq[max_idx]
    if max_freq > (len(s) + 1) // 2:
        return ""
    
    res = [''] * len(s)
    idx = 0
    max_char = chr(max_idx + ord('a'))

    while freq[max_idx] > 0:
        res[idx] = max_char
        idx += 2
        freq[max_idx] -= 1
    
    for i in range(26):
        while freq[i] > 0:
            if idx >= len(s):
                idx = 1
            res[idx] = chr(i + ord('a'))
            idx += 2
            freq[i] -= 1
    
    return ''.join(res)

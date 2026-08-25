func longestCommonPrefix(strs []string) string {
    prefix := strs[0]

    for i := 1; i < len(strs); i++ {
        j := 0
        minLen := len(prefix)
        if len(strs[i]) < minLen {
            minLen = len(strs[i])
        }

        for j < minLen {
            if prefix[j] != strs[i][j] {
                break
            }
            j++
        }
        prefix = prefix[:j]
    }
    return prefix
}

You are given a **0-indexed** binary string `s` and two integers `minJump` and `maxJump`. In the beginning, you are standing at index `0`, which is equal to `'0'`. You can move from index `i` to index `j` if the following conditions are fulfilled:

- `i + minJump <= j <= min(i + maxJump, s.length - 1)`, and
- `s[j] == '0'`.

Return `true` if you can reach index `s.length - 1` in `s`, or `false` otherwise.

**Example 1:**

```java
Input: s = "00110010", minJump = 2, maxJump = 4

Output: true
```

Explanation: The order of jumps is: indices 0 -> 4 -> 7.

**Example 2:**

```java
Input: s = "0010", minJump = 1, maxJump = 1

Output: false
```

**Constraints:**
* `2 <= s.length <= 100,000`
* `s[i]` is either `'0'` or `'1'`.
* `s[0] == '0'`
* `1 <= minJump <= maxJump < s.length`

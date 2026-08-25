You are given two integers `n` and `x`. You have to construct an array of positive integers `nums` of size `n` where for every `0 <= i < n - 1`, `nums[i + 1]` is **greater than** `nums[i]`, and the result of the **bitwise AND** operation between all elements of `nums` is `x`.

Return the **minimum** possible value of `nums[n - 1]`.

**Example 1:**

```java
Input: n = 3, x = 2

Output: 6
```

Explanation: nums can be [2,3,6].

**Example 2:**

```java
Input: n = 5, x = 3

Output: 19
```

Explanation: nums can be [3,7,11,15,19].

**Constraints:**
* `1 <= n, x <= 100,000,000`

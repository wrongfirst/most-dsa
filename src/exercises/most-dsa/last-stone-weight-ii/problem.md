You are given an array of integers `stones` where `stones[i]` is the weight of the `ith` stone.

We are playing a game with the stones. On each turn, we choose any two stones and smash them together. Suppose the stones have weights `x` and `y` with `x <= y`. The result of this smash is:

- If `x == y`, both stones are destroyed, and
- If `x != y`, the stone of weight `x` is destroyed, and the stone of weight `y` has new weight `y - x`.

At the end of the game, there is **at most one** stone left.

Return the **smallest** possible weight of the left stone. If there are no stones left, return `0`.

<br>

**Example 1:**

```java
Input: stones = [2,4,1,5,6,3]

Output: 1
```

Explanation: 
1. We smash `2` and `1` which makes the array `[1,4,5,6,3]`
2. We smash `4` and `3` which makes the array `[1,1,5,6]`
3. We smash `5` and `6` which makes the array `[1,1,1]`
4. We smash `1` and `1` which makes the array `[1]`

<br>

**Example 2:**

```java
Input: stones = [4,4,1,7,10]

Output: 2
```

<br>

**Constraints:**
* `1 <= stones.length <= 30`
* `1 <= stones[i] <= 100`


<br>

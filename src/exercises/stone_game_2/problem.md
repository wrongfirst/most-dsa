Alice and Bob continue their games with piles of stones. There are a number of piles arranged in a row, and each pile has a positive integer number of stones `piles[i]`. The objective of the game is to end with the most stones.

Alice and Bob take turns, with Alice starting first.

On each player's turn, that player can take all the stones in the **first** `X` remaining piles, where `1 <= X <= 2M`. Then, we set `M = max(M, X)`. Initially, `M = 1`.

The game continues until all the stones have been taken.

Assuming Alice and Bob play optimally, return the **maximum** number of stones Alice can get.

**Example 1:**

```java
Input: piles = [3,1,2,5,7]

Output: 10
```

Explanation: Alice takes first pile, then Bob takes the second pile, then Alice takes the next two piles and Bob takes the last remaining pile. This makes Alice's score 3 + 2 + 5 = 10, which is the maximum Alice can get.

**Example 2:**

```java
Input: piles = [4,3,2,5,10]

Output: 11
```

**Constraints:**
* `1 <= piles.length <= 100`
* `1 <= piles[i] <= 10,000`

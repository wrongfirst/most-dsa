You are given an array `nums` consisting of `n` elements where each element is an integer representing a color:
- `0` represents red
- `1` represents white
- `2` represents blue

Your task is to **sort the array in-place** such that elements of the same color are grouped together and arranged in the order: red (0), white (1), and then blue (2).

You **must not** use any built-in sorting functions to solve this problem.

<br>

**Example 1:**

```java
Input: nums = [1,0,1,2]

Output: [0,1,1,2]
```

<br>

**Example 2:**

```java
Input: nums = [2,1,0]

Output: [0,1,2]
```

<br>

**Constraints:**
* `1 <= nums.length <= 300`.
* `0 <= nums[i] <= 2`.

<br>

**Follow up:** Could you come up with a one-pass algorithm using only constant extra space?


<br>

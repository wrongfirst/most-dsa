You are given an integer array `arr`, return the length of a maximum size turbulent subarray of `arr`.

A subarray is **turbulent** if the comparison sign flips between each adjacent pair of elements in the subarray.

More formally, a subarray `[arr[i], arr[i + 1], ..., arr[j]]` of `arr` is said to be turbulent if and only if:

- For `i <= k < j`:
    - `arr[k] > arr[k + 1]` when `k` is odd, and
    - `arr[k] < arr[k + 1]` when `k` is even.

- Or, for `i <= k < j`:
    - `arr[k] > arr[k + 1]` when `k` is even, and
    - `arr[k] < arr[k + 1]` when `k` is odd.

**Example 1:**

```java
Input: arr = [2,4,3,2,2,5,1,4]

Output: 4
```

Explanation: The longest turbulent subarray is [2,5,1,4].

**Example 2:**

```java
Input: arr = [1,1,2]

Output: 2
```

**Constraints:**
* `1 <= arr.length <= 40,000`
* `0 <= arr[i] <= 1,000,000,000`

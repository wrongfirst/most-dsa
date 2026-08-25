A tree is an undirected graph in which any two vertices are connected by exactly one path. In other words, any connected graph without simple cycles is a tree.

You are given a tree of `n` nodes labelled from `0` to `n - 1`, and an array of `n - 1` `edges` where `edges[i] = [a[i], b[i]]` indicates that there is an undirected edge between the two nodes `a[i]` and `b[i]` in the tree, you can choose any node of the tree as the root. When you select a node `x` as the root, the result tree has height `h`. Among all possible rooted trees, those with minimum height (i.e. `min(h)`)  are called **minimum height trees** (MHTs).

Return a list of all **MHTs'** root labels. You can return the answer in **any order**.

The **height** of a rooted tree is the number of edges on the longest downward path between the root and a leaf.

**Example 1:**

![](https://imagedelivery.net/CLfkmk9Wzy8_9HRyug4EVA/ac35554c-7825-4b2f-02d7-41cc8cb38400/public)

```java
Input: n = 5, edges = [[0,1],[3,1],[2,3],[4,1]]

Output: [1,3]
```

Explanation: As shown, the trees with root labels `[1,3]` are MHT's with height of `2`.

**Example 2:**

![](https://imagedelivery.net/CLfkmk9Wzy8_9HRyug4EVA/87bb8fe7-085c-43c9-ccac-dd6719029f00/public)

```java
Input: n = 4, edges = [[1,0],[2,0],[3,0]]

Output: [0]
```

Explanation: As shown, the tree with root label `[0]` is MHT with height of `1`.

**Constraints:**
* `1 <= n <= 20,000`
* `edges.length == n - 1`
* `0 <= a[i], b[i] < n`
* `a[i] != b[i]`
* All the pairs `(a[i], b[i])` are **distinct**.
* The given input is **guaranteed** to be a tree and there will be **no repeated** edges.

/**
 * Definition for a QuadTree node.
 * type Node struct {
 *     Val bool
 *     IsLeaf bool
 *     TopLeft *Node
 *     TopRight *Node
 *     BottomLeft *Node
 *     BottomRight *Node
 * }
 */

func construct(grid [][]int) *Node {
	leafNodes := map[int]*Node{
		0: {Val: false, IsLeaf: true},
		1: {Val: true, IsLeaf: true},
	}

	var dfs func(n, r, c int) *Node
	dfs = func(n, r, c int) *Node {
		if n == 1 {
			return leafNodes[grid[r][c]]
		}

		n /= 2
		topLeft := dfs(n, r, c)
		topRight := dfs(n, r, c+n)
		bottomLeft := dfs(n, r+n, c)
		bottomRight := dfs(n, r+n, c+n)

		if topLeft.IsLeaf && topRight.IsLeaf &&
			bottomLeft.IsLeaf && bottomRight.IsLeaf &&
			topLeft.Val == topRight.Val && topLeft.Val == bottomLeft.Val && topLeft.Val == bottomRight.Val {
			return topLeft
		}

		return &Node{Val: false, IsLeaf: false, TopLeft: topLeft, TopRight: topRight, BottomLeft: bottomLeft, BottomRight: bottomRight}
	}

	return dfs(len(grid), 0, 0)
}

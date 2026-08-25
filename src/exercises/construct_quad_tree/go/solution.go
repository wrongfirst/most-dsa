type QuadNode struct {
	Val         bool
	IsLeaf      bool
	TopLeft     *QuadNode
	TopRight    *QuadNode
	BottomLeft  *QuadNode
	BottomRight *QuadNode
}

func construct(grid [][]int) *QuadNode {
	leafNodes := map[int]*QuadNode{
		0: {Val: false, IsLeaf: true},
		1: {Val: true, IsLeaf: true},
	}

	var dfs func(n, r, c int) *QuadNode
	dfs = func(n, r, c int) *QuadNode {
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

		return &QuadNode{Val: false, IsLeaf: false, TopLeft: topLeft, TopRight: topRight, BottomLeft: bottomLeft, BottomRight: bottomRight}
	}

	return dfs(len(grid), 0, 0)
}


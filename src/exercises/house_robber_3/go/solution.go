/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
 
func rob(root *TreeNode) int {
	var dfs func(root *TreeNode) [2]int
	dfs = func(root *TreeNode) [2]int {
		if root == nil {
			return [2]int{0, 0}
		}

		leftPair := dfs(root.Left)
		rightPair := dfs(root.Right)

		withRoot := root.Val + leftPair[1] + rightPair[1]
		withoutRoot := max(leftPair[0], leftPair[1]) + max(rightPair[0], rightPair[1])

		return [2]int{withRoot, withoutRoot}
	}

	result := dfs(root)
	return max(result[0], result[1])
}

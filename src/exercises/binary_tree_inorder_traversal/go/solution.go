/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */

func inorderTraversal(root *TreeNode) []int {
	res := []int{}
	cur := root

	for cur != nil {
		if cur.Left == nil {
			res = append(res, cur.Val)
			cur = cur.Right
		} else {
			prev := cur.Left

			for prev.Right != nil && prev.Right != cur {
				prev = prev.Right
			}

			if prev.Right == nil {
				prev.Right = cur
				cur = cur.Left
			} else {
				prev.Right = nil
				res = append(res, cur.Val)
				cur = cur.Right
			}
		}
	}

	return res
}

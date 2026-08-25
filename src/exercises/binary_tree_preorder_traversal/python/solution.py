# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
def preorderTraversal(root: Optional[TreeNode]) -> list[int]:
    res = []
    cur = root

    while cur:
        if not cur.left:
            res.append(cur.val)
            cur = cur.right
        else:
            prev = cur.left
            while prev.right and prev.right != cur:
                prev = prev.right

            if not prev.right:
                res.append(cur.val)
                prev.right = cur
                cur = cur.left
            else:
                prev.right = None
                cur = cur.right

    return res

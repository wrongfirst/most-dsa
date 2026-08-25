# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
def removeLeafNodes(root: Optional[TreeNode], target: int) -> Optional[TreeNode]:
    if not root:
        return None

    root.left = removeLeafNodes(root.left, target)
    root.right = removeLeafNodes(root.right, target)

    if not root.left and not root.right and root.val == target:
        return None

    return root

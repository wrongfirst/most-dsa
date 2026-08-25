# Definition for a QuadTree node.
class Node:
    def __init__(
        self,
        val: bool,
        isLeaf: bool,
        topLeft: Optional['Node'] = None,
        topRight: Optional['Node'] = None,
        bottomLeft: Optional['Node'] = None,
        bottomRight: Optional['Node'] = None,
    ):
        self.val = val
        self.isLeaf = isLeaf
        self.topLeft = topLeft
        self.topRight = topRight
        self.bottomLeft = bottomLeft
        self.bottomRight = bottomRight

def construct(grid: list[list[int]]) -> Optional[Node]:
    leafNodes = {
        0: Node(False, True),
        1: Node(True, True)
    }

    def dfs(n: int, r: int, c: int) -> Node:
        if n == 1:
            return leafNodes[grid[r][c]]

        n //= 2
        topLeft = dfs(n, r, c)
        topRight = dfs(n, r, c + n)
        bottomLeft = dfs(n, r + n, c)
        bottomRight = dfs(n, r + n, c + n)

        if (topLeft.isLeaf and topRight.isLeaf and 
            bottomLeft.isLeaf and bottomRight.isLeaf and
            topLeft.val == topRight.val == bottomLeft.val == bottomRight.val):
            return topLeft

        return Node(False, False, topLeft, topRight, bottomLeft, bottomRight)

    return dfs(len(grid), 0, 0)

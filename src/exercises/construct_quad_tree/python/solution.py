# Definition for a QuadTree node.
class QuadNode:
    def __init__(
        self,
        val: bool,
        isLeaf: bool,
        topLeft: Optional['QuadNode'] = None,
        topRight: Optional['QuadNode'] = None,
        bottomLeft: Optional['QuadNode'] = None,
        bottomRight: Optional['QuadNode'] = None,
    ):
        self.val = val
        self.isLeaf = isLeaf
        self.topLeft = topLeft
        self.topRight = topRight
        self.bottomLeft = bottomLeft
        self.bottomRight = bottomRight

def construct(grid: list[list[int]]) -> Optional[QuadNode]:
    leafNodes = {
        0: QuadNode(False, True),
        1: QuadNode(True, True)
    }

    def dfs(n: int, r: int, c: int) -> QuadNode:
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

        return QuadNode(False, False, topLeft, topRight, bottomLeft, bottomRight)

    return dfs(len(grid), 0, 0)

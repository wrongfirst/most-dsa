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
    pass

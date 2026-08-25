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
    ...

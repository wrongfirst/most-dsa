class UnionFind:
    def __init__(self, n: int):
        self.Parent = list(range(n + 1))
        self.Size = [1] * (n + 1)
        
    def find(self, node: int) -> int:
        if self.Parent[node] != node:
            self.Parent[node] = self.find(self.Parent[node])
        return self.Parent[node]
    
    def union(self, u: int, v: int) -> bool:
        pu = self.find(u)
        pv = self.find(v)
        if pu == pv:
            return False
        if self.Size[pu] < self.Size[pv]:
            pu, pv = pv, pu
        self.Size[pu] += self.Size[pv]
        self.Parent[pv] = pu
        return True

def findCriticalAndPseudoCriticalEdges(n: int, edges: list[list[int]]) -> list[list[int]]:
    mst: list[list[tuple[int, int]]] = [[] for _ in range(n)]
    mst_edges: list[int] = []
    
    edge_list = [(w, u, v, i) for i, (u, v, w) in enumerate(edges)]
    edge_list.sort()
    
    uf = UnionFind(n)
    for w, u, v, i in edge_list:
        if uf.union(u, v):
            mst[u].append((v, i))
            mst[v].append((u, i))
            mst_edges.append(i)
    
    def dfs(node: int, dst: int, path: list[int]) -> bool:
        for next_node, ind in mst[node]:
            if path and ind == path[-1]:
                continue
            path.append(ind)
            if next_node == dst or dfs(next_node, dst, path):
                return True
            path.pop()
        return False
    
    pseudo: set[int] = set()
    mst_set: set[int] = set(mst_edges)
    for ind in range(len(edges)):
        if ind in mst_set:
            continue
        path: list[int] = []
        dst = edges[ind][1]
        dfs(edges[ind][0], dst, path)
        for i in path:
            if edges[i][2] == edges[ind][2]:
                pseudo.add(i)
                pseudo.add(ind)
    
    return [list(mst_set - pseudo), list(pseudo)]

class UnionFind:
    def __init__(self, n):
        self.Parent = list(range(n + 1))
        self.Size = [1] * (n + 1)
        
    def find(self, node):
        if self.Parent[node] != node:
            self.Parent[node] = self.find(self.Parent[node])
        return self.Parent[node]
    
    def union(self, u, v):
        pu = self.find(u)
        pv = self.find(v)
        if pu == pv:
            return False
        if self.Size[pu] < self.Size[pv]:
            pu, pv = pv, pu
        self.Size[pu] += self.Size[pv]
        self.Parent[pv] = pu
        return True

class Solution:
    def findCriticalAndPseudoCriticalEdges(self, n: int, edges: List[List[int]]) -> List[List[int]]:
        mst = [[] for _ in range(n)]
        mstEdge = []
        
        edge_list = [(w, u, v, i) for i, (u, v, w) in enumerate(edges)]
        edge_list.sort()
        
        uf = UnionFind(n)
        for w, u, v, i in edge_list:
            if uf.union(u, v):
                mst[u].append((v, i))
                mst[v].append((u, i))
                mstEdge.append(i)
        
        def dfs(node):
            for next, ind in mst[node]:
                if path and ind == path[-1]:
                    continue
                path.append(ind)
                if next == dst or dfs(next):
                    return True
                path.pop()
            return False
        
        pseudo, mstEdge = set(), set(mstEdge)
        for ind in range(len(edges)):
            if ind in mstEdge:
                continue
            path, dst = [], edges[ind][1]
            dfs(edges[ind][0])
            for i in path:
                if edges[i][2] == edges[ind][2]:
                    pseudo.add(i)
                    pseudo.add(ind)
        
        return [list(mstEdge - pseudo), list(pseudo)]

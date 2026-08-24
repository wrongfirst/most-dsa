type UnionFind struct {
	Parent []int
	Size   []int
}

func newUnionFind(n int) *UnionFind {
	parent := make([]int, n+1)
	size := make([]int, n+1)
	for i := 0; i <= n; i++ {
		parent[i] = i
		size[i] = 1
	}
	return &UnionFind{Parent: parent, Size: size}
}

func (uf *UnionFind) find(node int) int {
	if uf.Parent[node] != node {
		uf.Parent[node] = uf.find(uf.Parent[node])
	}
	return uf.Parent[node]
}

func (uf *UnionFind) union(u, v int) bool {
	pu := uf.find(u)
	pv := uf.find(v)
	if pu == pv {
		return false
	}
	if uf.Size[pu] < uf.Size[pv] {
		pu, pv = pv, pu
	}
	uf.Size[pu] += uf.Size[pv]
	uf.Parent[pv] = pu
	return true
}

type Edge struct {
	w   int
	u   int
	v   int
	idx int
}

type Neighbor struct {
	node int
	idx  int
}

func findCriticalAndPseudoCriticalEdges(n int, edges [][]int) [][]int {
	mst := make([][]Neighbor, n)
	for i := range mst {
		mst[i] = []Neighbor{}
	}
	mstEdge := []int{}

	edgeList := []Edge{}
	for i, edge := range edges {
		edgeList = append(edgeList, Edge{w: edge[2], u: edge[0], v: edge[1], idx: i})
	}

	sort.Slice(edgeList, func(i, j int) bool {
		return edgeList[i].w < edgeList[j].w
	})

	uf := newUnionFind(n)
	for _, e := range edgeList {
		if uf.union(e.u, e.v) {
			mst[e.u] = append(mst[e.u], Neighbor{node: e.v, idx: e.idx})
			mst[e.v] = append(mst[e.v], Neighbor{node: e.u, idx: e.idx})
			mstEdge = append(mstEdge, e.idx)
		}
	}

	var path []int
	var dst int

	var dfs func(node int) bool
	dfs = func(node int) bool {
		for _, neighbor := range mst[node] {
			next, ind := neighbor.node, neighbor.idx
			if len(path) > 0 && ind == path[len(path)-1] {
				continue
			}
			path = append(path, ind)
			if next == dst || dfs(next) {
				return true
			}
			path = path[:len(path)-1]
		}
		return false
	}

	pseudo := make(map[int]bool)
	mstEdgeSet := make(map[int]bool)
	for _, idx := range mstEdge {
		mstEdgeSet[idx] = true
	}

	for ind := 0; ind < len(edges); ind++ {
		if mstEdgeSet[ind] {
			continue
		}
		path = []int{}
		dst = edges[ind][1]
		dfs(edges[ind][0])

		for _, i := range path {
			if edges[i][2] == edges[ind][2] {
				pseudo[i] = true
				pseudo[ind] = true
			}
		}
	}

	critical := []int{}
	for _, idx := range mstEdge {
		if !pseudo[idx] {
			critical = append(critical, idx)
		}
	}

	pseudoList := []int{}
	for idx := range pseudo {
		pseudoList = append(pseudoList, idx)
	}

	return [][]int{critical, pseudoList}
}

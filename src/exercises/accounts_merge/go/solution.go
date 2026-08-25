type UnionFind struct {
	par  []int
	rank []int
}

func newUnionFind(n int) *UnionFind {
	par := make([]int, n)
	rank := make([]int, n)
	for i := 0; i < n; i++ {
		par[i] = i
		rank[i] = 1
	}
	return &UnionFind{par: par, rank: rank}
}

func (uf *UnionFind) find(x int) int {
	for x != uf.par[x] {
		uf.par[x] = uf.par[uf.par[x]]
		x = uf.par[x]
	}
	return x
}

func (uf *UnionFind) union(x1, x2 int) bool {
	p1, p2 := uf.find(x1), uf.find(x2)
	if p1 == p2 {
		return false
	}

	if uf.rank[p1] > uf.rank[p2] {
		uf.par[p2] = p1
		uf.rank[p1] += uf.rank[p2]
	} else {
		uf.par[p1] = p2
		uf.rank[p2] += uf.rank[p1]
	}

	return true
}

func accountsMerge(accounts [][]string) [][]string {
	uf := newUnionFind(len(accounts))
	emailToAcc := make(map[string]int)

	for i, a := range accounts {
		for j := 1; j < len(a); j++ {
			e := a[j]
			if idx, exists := emailToAcc[e]; exists {
				uf.union(i, idx)
			} else {
				emailToAcc[e] = i
			}
		}
	}

	emailGroup := make(map[int][]string)
	for e, i := range emailToAcc {
		leader := uf.find(i)
		emailGroup[leader] = append(emailGroup[leader], e)
	}

	res := [][]string{}
	for i, emails := range emailGroup {
		name := accounts[i][0]
		sort.Strings(emails)
		account := append([]string{name}, emails...)
		res = append(res, account)
	}

	return res
}

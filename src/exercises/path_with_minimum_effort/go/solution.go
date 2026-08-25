type Item struct {
	diff int
	r    int
	c    int
}

func minimumEffortPath(heights [][]int) int {
	ROWS, COLS := len(heights), len(heights[0])
	minHeap := &MinHeap{}
	heap.Init(minHeap)
	heap.Push(minHeap, Item{diff: 0, r: 0, c: 0})

	visit := make(map[[2]int]bool)
	directions := [][2]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}

	for minHeap.Len() > 0 {
		item := heap.Pop(minHeap).(Item)
		diff, r, c := item.diff, item.r, item.c

		if visit[[2]int{r, c}] {
			continue
		}
		visit[[2]int{r, c}] = true

		if r == ROWS-1 && c == COLS-1 {
			return diff
		}

		for _, dir := range directions {
			newR, newC := r+dir[0], c+dir[1]

			if newR < 0 || newC < 0 || newR >= ROWS || newC >= COLS || visit[[2]int{newR, newC}] {
				continue
			}

			newDiff := max(diff, abs(heights[r][c]-heights[newR][newC]))
			heap.Push(minHeap, Item{diff: newDiff, r: newR, c: newC})
		}
	}

	return 0
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}


type MinHeap []Item

func (h MinHeap) Len() int           { return len(h) }
func (h MinHeap) Less(i, j int) bool { return h[i].diff < h[j].diff }
func (h MinHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *MinHeap) Push(x interface{}) {
	*h = append(*h, x.(Item))
}

func (h *MinHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

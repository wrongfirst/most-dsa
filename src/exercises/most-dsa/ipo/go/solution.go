func findMaximizedCapital(k int, w int, profits []int, capital []int) int {
	n := len(profits)
	indices := make([]int, n)
	for i := 0; i < n; i++ {
		indices[i] = i
	}

	sort.Slice(indices, func(i, j int) bool {
		return capital[indices[i]] < capital[indices[j]]
	})

	maxProfit := &MaxHeap{}
	heap.Init(maxProfit)
	idx := 0

	for i := 0; i < k; i++ {
		for idx < n && capital[indices[idx]] <= w {
			heap.Push(maxProfit, profits[indices[idx]])
			idx++
		}

		if maxProfit.Len() == 0 {
			break
		}

		w += heap.Pop(maxProfit).(int)
	}

	return w
}

type MaxHeap []int

func (h MaxHeap) Len() int           { return len(h) }
func (h MaxHeap) Less(i, j int) bool { return h[i] > h[j] }
func (h MaxHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *MaxHeap) Push(x interface{}) {
	*h = append(*h, x.(int))
}
func (h *MaxHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

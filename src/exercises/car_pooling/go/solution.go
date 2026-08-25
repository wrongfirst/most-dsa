type Trip struct {
	end      int
	numPass int
}

func carPooling(trips [][]int, capacity int) bool {
	sort.Slice(trips, func(i, j int) bool {
		return trips[i][1] < trips[j][1]
	})

	minHeap := &MinHeap{}
	heap.Init(minHeap)
	curPass := 0

	for _, trip := range trips {
		numPass := trip[0]
		start := trip[1]
		end := trip[2]

		for minHeap.Len() > 0 && (*minHeap)[0].end <= start {
			curPass -= heap.Pop(minHeap).(Trip).numPass
		}

		curPass += numPass
		if curPass > capacity {
			return false
		}

		heap.Push(minHeap, Trip{end: end, numPass: numPass})
	}

	return true
}

type MinHeap []Trip

func (h MinHeap) Len() int           { return len(h) }
func (h MinHeap) Less(i, j int) bool { return h[i].end < h[j].end }
func (h MinHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *MinHeap) Push(x interface{}) {
	*h = append(*h, x.(Trip))
}
func (h *MinHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

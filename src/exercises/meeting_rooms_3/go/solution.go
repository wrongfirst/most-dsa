func mostBooked(n int, meetings [][]int) int {
	sort.Slice(meetings, func(i, j int) bool {
		return meetings[i][0] < meetings[j][0]
	})
	available := &MinHeap{}
	heap.Init(available)
	count := make([]int, n)
	for i := 0; i < n; i++ {
		heap.Push(available, Room{0, i})
	}
	for _, meeting := range meetings {
		start := meeting[0]
		end := meeting[1]
		for available.Len() > 0 && (*available)[0].endTime < start {
			room := heap.Pop(available).(Room)
			heap.Push(available, Room{start, room.id})
		}

		room := heap.Pop(available).(Room)
		heap.Push(available, Room{room.endTime + (end - start), room.id})
		count[room.id]++
	}
	maxCount := count[0]
	result := 0
	for i := 1; i < n; i++ {
		if count[i] > maxCount {
			maxCount = count[i]
			result = i
		}
	}
	return result
}

type Room struct {
	endTime int
	id      int
}

type MinHeap []Room

func (h MinHeap) Len() int { return len(h) }
func (h MinHeap) Less(i, j int) bool {
	if h[i].endTime == h[j].endTime {
		return h[i].id < h[j].id
	}
	return h[i].endTime < h[j].endTime
}
func (h MinHeap) Swap(i, j int) { h[i], h[j] = h[j], h[i] }

func (h *MinHeap) Push(x interface{}) {
	*h = append(*h, x.(Room))
}

func (h *MinHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

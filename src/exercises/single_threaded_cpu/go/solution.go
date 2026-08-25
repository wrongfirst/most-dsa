type Task struct {
	procTime int
	index    int
}

func getOrder(tasks [][]int) []int {
	for i := range tasks {
		tasks[i] = append(tasks[i], i)
	}

	sort.Slice(tasks, func(i, j int) bool {
		return tasks[i][0] < tasks[j][0]
	})

	res := []int{}
	minHeap := &MinHeap{}
	heap.Init(minHeap)

	i := 0
	time := tasks[0][0]

	for minHeap.Len() > 0 || i < len(tasks) {
		for i < len(tasks) && time >= tasks[i][0] {
			heap.Push(minHeap, Task{procTime: tasks[i][1], index: tasks[i][2]})
			i++
		}

		if minHeap.Len() == 0 {
			time = tasks[i][0]
		} else {
			task := heap.Pop(minHeap).(Task)
			time += task.procTime
			res = append(res, task.index)
		}
	}

	return res
}

type MinHeap []Task

func (h MinHeap) Len() int { return len(h) }

func (h MinHeap) Less(i, j int) bool {
	if h[i].procTime == h[j].procTime {
		return h[i].index < h[j].index
	}
	return h[i].procTime < h[j].procTime
}

func (h MinHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *MinHeap) Push(x interface{}) { *h = append(*h, x.(Task)) }

func (h *MinHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

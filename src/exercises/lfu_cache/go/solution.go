type ListNode struct {
	val  int
	prev *ListNode
	next *ListNode
}

type LinkedList struct {
	left  *ListNode
	right *ListNode
	m     map[int]*ListNode
}

func newLinkedList() *LinkedList {
	left := &ListNode{val: 0}
	right := &ListNode{val: 0, prev: left}
	left.next = right
	return &LinkedList{
		left:  left,
		right: right,
		m:     make(map[int]*ListNode),
	}
}

func (this *LinkedList) length() int {
	return len(this.m)
}

func (this *LinkedList) pushRight(val int) {
	node := &ListNode{val: val, prev: this.right.prev, next: this.right}
	this.m[val] = node
	this.right.prev = node
	node.prev.next = node
}

func (this *LinkedList) pop(val int) {
	if node, exists := this.m[val]; exists {
		next, prev := node.next, node.prev
		next.prev = prev
		prev.next = next
		delete(this.m, val)
	}
}

func (this *LinkedList) popLeft() int {
	res := this.left.next.val
	this.pop(this.left.next.val)
	return res
}

func (this *LinkedList) update(val int) {
	this.pop(val)
	this.pushRight(val)
}

type LFUCache struct {
	cap      int
	lfuCnt   int
	valMap   map[int]int
	countMap map[int]int
	listMap  map[int]*LinkedList
}

func Constructor(capacity int) LFUCache {
	return LFUCache{
		cap:      capacity,
		lfuCnt:   0,
		valMap:   make(map[int]int),
		countMap: make(map[int]int),
		listMap:  make(map[int]*LinkedList),
	}
}

func (this *LFUCache) counter(key int) {
	cnt := this.countMap[key]
	this.countMap[key]++

	if this.listMap[cnt] == nil {
		this.listMap[cnt] = newLinkedList()
	}
	this.listMap[cnt].pop(key)

	if this.listMap[cnt+1] == nil {
		this.listMap[cnt+1] = newLinkedList()
	}
	this.listMap[cnt+1].pushRight(key)

	if cnt == this.lfuCnt && this.listMap[cnt].length() == 0 {
		this.lfuCnt++
	}
}

func (this *LFUCache) Get(key int) int {
	if _, exists := this.valMap[key]; !exists {
		return -1
	}
	this.counter(key)
	return this.valMap[key]
}

func (this *LFUCache) Put(key int, value int) {
	if this.cap == 0 {
		return
	}

	if _, exists := this.valMap[key]; !exists && len(this.valMap) == this.cap {
		if this.listMap[this.lfuCnt] == nil {
			this.listMap[this.lfuCnt] = newLinkedList()
		}
		res := this.listMap[this.lfuCnt].popLeft()
		delete(this.valMap, res)
		delete(this.countMap, res)
	}

	this.valMap[key] = value
	this.counter(key)
	this.lfuCnt = min(this.lfuCnt, this.countMap[key])
}

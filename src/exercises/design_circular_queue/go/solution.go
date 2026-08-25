type QueueNode struct {
	val  int
	next *QueueNode
}

type MyCircularQueue struct {
	space int
	left  *QueueNode
	right *QueueNode
}

func Constructor(k int) MyCircularQueue {
	left := &QueueNode{val: 0}
	return MyCircularQueue{
		space: k,
		left:  left,
		right: left,
	}
}

func (this *MyCircularQueue) EnQueue(value int) bool {
	if this.IsFull() {
		return false
	}

	cur := &QueueNode{val: value}

	if this.IsEmpty() {
		this.left.next = cur
		this.right = cur
	} else {
		this.right.next = cur
		this.right = cur
	}

	this.space--
	return true
}

func (this *MyCircularQueue) DeQueue() bool {
	if this.IsEmpty() {
		return false
	}

	this.left.next = this.left.next.next

	if this.left.next == nil {
		this.right = this.left
	}

	this.space++
	return true
}

func (this *MyCircularQueue) Front() int {
	if this.IsEmpty() {
		return -1
	}
	return this.left.next.val
}

func (this *MyCircularQueue) Rear() int {
	if this.IsEmpty() {
		return -1
	}
	return this.right.val
}

func (this *MyCircularQueue) IsEmpty() bool {
	return this.left.next == nil
}

func (this *MyCircularQueue) IsFull() bool {
	return this.space == 0
}


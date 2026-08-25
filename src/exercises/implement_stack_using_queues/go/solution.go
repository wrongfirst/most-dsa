type QueueNode struct {
	value int
	next  *QueueNode
}

type MyStack struct {
	q *QueueNode
}

func Constructor() MyStack {
	return MyStack{q: nil}
}

func (this *MyStack) Push(x int) {
	this.q = &QueueNode{value: x, next: this.q}
}

func (this *MyStack) Pop() int {
	top := this.q.value
	this.q = this.q.next
	return top
}

func (this *MyStack) Top() int {
	return this.q.value
}

func (this *MyStack) Empty() bool {
	return this.q == nil
}

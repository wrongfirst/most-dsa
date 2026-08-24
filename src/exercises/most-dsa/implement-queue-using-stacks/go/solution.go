type MyQueue struct {
	s1 []int
	s2 []int
}

func Constructor() MyQueue {
	return MyQueue{
		s1: []int{},
		s2: []int{},
	}
}

func (this *MyQueue) Push(x int) {
	this.s1 = append(this.s1, x)
}

func (this *MyQueue) Pop() int {
	if len(this.s2) == 0 {
		for len(this.s1) > 0 {
			this.s2 = append(this.s2, this.s1[len(this.s1)-1])
			this.s1 = this.s1[:len(this.s1)-1]
		}
	}
	result := this.s2[len(this.s2)-1]
	this.s2 = this.s2[:len(this.s2)-1]
	return result
}

func (this *MyQueue) Peek() int {
	if len(this.s2) == 0 {
		for len(this.s1) > 0 {
			this.s2 = append(this.s2, this.s1[len(this.s1)-1])
			this.s1 = this.s1[:len(this.s1)-1]
		}
	}
	return this.s2[len(this.s2)-1]
}

func (this *MyQueue) Empty() bool {
	return max(len(this.s1), len(this.s2)) == 0
}

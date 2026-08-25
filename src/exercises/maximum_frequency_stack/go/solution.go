type FreqStack struct {
	cnt    map[int]int
	stacks [][]int
}

func Constructor() FreqStack {
	return FreqStack{
		cnt:    map[int]int{},
		stacks: [][]int{{}},
	}
}

func (this *FreqStack) Push(val int) {
	valCnt := 1 + this.cnt[val]
	this.cnt[val] = valCnt

	if valCnt == len(this.stacks) {
		this.stacks = append(this.stacks, []int{})
	}

	this.stacks[valCnt] = append(this.stacks[valCnt], val)
}

func (this *FreqStack) Pop() int {
	res := this.stacks[len(this.stacks)-1][len(this.stacks[len(this.stacks)-1])-1]
	this.stacks[len(this.stacks)-1] = this.stacks[len(this.stacks)-1][:len(this.stacks[len(this.stacks)-1])-1]
	this.cnt[res]--

	if len(this.stacks[len(this.stacks)-1]) == 0 {
		this.stacks = this.stacks[:len(this.stacks)-1]
	}

	return res
}

type StockSpanner struct {
	stack [][2]int
}

func Constructor() StockSpanner {
	return StockSpanner{
		stack: [][2]int{},
	}
}

func (this *StockSpanner) Next(price int) int {
	span := 1

	for len(this.stack) > 0 && this.stack[len(this.stack)-1][0] <= price {
		span += this.stack[len(this.stack)-1][1]
		this.stack = this.stack[:len(this.stack)-1]
	}

	this.stack = append(this.stack, [2]int{price, span})
	return span
}

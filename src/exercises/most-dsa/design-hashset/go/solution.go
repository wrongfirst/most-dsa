type MyHashSet struct {
    set []int
}

func Constructor() MyHashSet {
    return MyHashSet{
        set: make([]int, 31251),
    }
}

func (this *MyHashSet) Add(key int) {
    this.set[key/32] |= this.getMask(key)
}

func (this *MyHashSet) Remove(key int) {
    if this.Contains(key) {
        this.set[key/32] ^= this.getMask(key)
    }
}

func (this *MyHashSet) Contains(key int) bool {
    return this.set[key/32]&this.getMask(key) != 0
}

func (this *MyHashSet) getMask(key int) int {
    return 1 << (key % 32)
}

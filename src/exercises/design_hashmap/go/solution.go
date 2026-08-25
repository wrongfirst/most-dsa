type ListNode struct {
    key  int
    val  int
    next *ListNode
}

type MyHashMap struct {
    hashMap []*ListNode
}

func Constructor() MyHashMap {
    hashMap := make([]*ListNode, 1000)
    for i := 0; i < 1000; i++ {
        hashMap[i] = &ListNode{key: -1, val: -1}
    }
    return MyHashMap{hashMap: hashMap}
}

func (this *MyHashMap) hash(key int) int {
    return key % len(this.hashMap)
}

func (this *MyHashMap) Put(key int, value int) {
    cur := this.hashMap[this.hash(key)]
    for cur.next != nil {
        if cur.next.key == key {
            cur.next.val = value
            return
        }
        cur = cur.next
    }
    cur.next = &ListNode{key: key, val: value}
}

func (this *MyHashMap) Get(key int) int {
    cur := this.hashMap[this.hash(key)].next
    for cur != nil {
        if cur.key == key {
            return cur.val
        }
        cur = cur.next
    }
    return -1
}

func (this *MyHashMap) Remove(key int) {
    cur := this.hashMap[this.hash(key)]
    for cur.next != nil {
        if cur.next.key == key {
            cur.next = cur.next.next
            return
        }
        cur = cur.next
    }
}

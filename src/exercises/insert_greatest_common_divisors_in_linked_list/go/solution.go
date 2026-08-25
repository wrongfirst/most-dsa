/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */

func insertGreatestCommonDivisors(head *ListNode) *ListNode {
    gcd := func(a int, b int) int {
		for b > 0 {
			a, b = b, a%b
		}
		return a
	}
	cur := head
	for cur.Next != nil {
		n1 := cur.Val
		n2 := cur.Next.Val
		cur.Next = &ListNode{gcd(n1, n2), cur.Next}
		cur = cur.Next.Next
	}
	return head
}

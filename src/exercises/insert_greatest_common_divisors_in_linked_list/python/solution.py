# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
def insertGreatestCommonDivisors(head: Optional[ListNode]) -> Optional[ListNode]:
    def gcd(a: int, b: int) -> int:
        while b > 0:
            a, b = b, a % b
        return a

    cur = head
    while cur and cur.next:
        n1, n2 = cur.val, cur.next.val
        cur.next = ListNode(gcd(n1, n2), cur.next)
        cur = cur.next.next

    return head

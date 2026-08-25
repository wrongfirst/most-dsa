# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
def reverseBetween(head: Optional[ListNode], left: int, right: int) -> Optional[ListNode]:
    if not head or left == right:
        return head

    dummy = ListNode(0, head)
    leftPrev: ListNode = dummy
    cur: Optional[ListNode] = head

    for _ in range(left - 1):
        if cur:
            leftPrev, cur = cur, cur.next

    prev: Optional[ListNode] = None
    for _ in range(right - left + 1):
        if cur:
            tmpNext = cur.next
            cur.next = prev
            prev, cur = cur, tmpNext

    if leftPrev.next:
        leftPrev.next.next = cur
    leftPrev.next = prev

    return dummy.next

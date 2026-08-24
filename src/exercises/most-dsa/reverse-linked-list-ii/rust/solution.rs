// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut left_prev = &mut dummy;

        // Move to the node before left position
        for _ in 0..left - 1 {
            left_prev = &mut left_prev.as_mut().unwrap().next;
        }

        // Extract the sublist to reverse
        let mut cur = left_prev.as_mut().unwrap().next.take();
        let mut prev: Option<Box<ListNode>> = None;

        // Reverse the sublist
        for _ in 0..right - left + 1 {
            let mut node = cur.unwrap();
            cur = node.next.take();
            node.next = prev;
            prev = Some(node);
        }

        // Connect the reversed sublist back
        // Find the tail of reversed sublist (which is original left node)
        let mut tail = &mut prev;
        while tail.as_ref().unwrap().next.is_some() {
            tail = &mut tail.as_mut().unwrap().next;
        }
        tail.as_mut().unwrap().next = cur;

        // Connect left_prev to head of reversed sublist
        left_prev.as_mut().unwrap().next = prev;

        dummy.unwrap().next
    }
}

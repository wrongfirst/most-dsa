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
    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b > 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            a
        }

        let mut head = head;
        let mut current = head.as_mut();

        while let Some(node) = current {
            if node.next.is_some() {
                let n1 = node.val;
                let n2 = node.next.as_ref().unwrap().val;
                let gcd_val = gcd(n1, n2);

                let mut new_node = ListNode::new(gcd_val);
                new_node.next = node.next.take();
                node.next = Some(Box::new(new_node));

                // Move to the node after the newly inserted one
                current = node.next.as_mut().unwrap().next.as_mut();
            } else {
                current = None;
            }
        }

        head
    }
}

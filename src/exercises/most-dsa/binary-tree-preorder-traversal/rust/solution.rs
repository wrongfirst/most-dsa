// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();

        if let Some(node) = root {
            stack.push(node);
        }

        while let Some(node) = stack.pop() {
            let node_ref = node.borrow();
            res.push(node_ref.val);

            if let Some(ref right) = node_ref.right {
                stack.push(right.clone());
            }
            if let Some(ref left) = node_ref.left {
                stack.push(left.clone());
            }
        }

        res
    }
}

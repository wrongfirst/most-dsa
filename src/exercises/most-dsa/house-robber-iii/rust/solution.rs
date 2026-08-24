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
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (with_root, without_root) = Self::dfs(&root);
        std::cmp::max(with_root, without_root)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match root {
            None => (0, 0),
            Some(node) => {
                let node_ref = node.borrow();
                let left_pair = Self::dfs(&node_ref.left);
                let right_pair = Self::dfs(&node_ref.right);

                let with_root = node_ref.val + left_pair.1 + right_pair.1;
                let without_root = std::cmp::max(left_pair.0, left_pair.1)
                    + std::cmp::max(right_pair.0, right_pair.1);

                (with_root, without_root)
            }
        }
    }
}

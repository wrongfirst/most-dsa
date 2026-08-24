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
    pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(node) => {
                let left = Self::remove_leaf_nodes(node.borrow().left.clone(), target);
                let right = Self::remove_leaf_nodes(node.borrow().right.clone(), target);

                node.borrow_mut().left = left.clone();
                node.borrow_mut().right = right.clone();

                // If this is now a leaf node and has the target value, delete it
                if left.is_none() && right.is_none() && node.borrow().val == target {
                    return None;
                }

                Some(node)
            }
        }
    }
}

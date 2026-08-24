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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut cur = root;

        while cur.is_some() {
            let cur_node = cur.as_ref().unwrap().borrow();
            if cur_node.left.is_none() {
                res.push(cur_node.val);
                let right = cur_node.right.clone();
                drop(cur_node);
                cur = right;
            } else {
                let mut prev = cur_node.left.clone();
                drop(cur_node);

                // Find the rightmost node in left subtree or the node that points back to cur
                loop {
                    let prev_node = prev.as_ref().unwrap().borrow();
                    if prev_node.right.is_none() {
                        break;
                    }
                    if Rc::ptr_eq(prev_node.right.as_ref().unwrap(), cur.as_ref().unwrap()) {
                        break;
                    }
                    let next = prev_node.right.clone();
                    drop(prev_node);
                    prev = next;
                }

                let prev_node = prev.as_ref().unwrap().borrow();
                if prev_node.right.is_none() {
                    // Make cur the right child of its inorder predecessor
                    drop(prev_node);
                    prev.as_ref().unwrap().borrow_mut().right = cur.clone();
                    let left = cur.as_ref().unwrap().borrow().left.clone();
                    cur = left;
                } else {
                    // Revert the changes
                    drop(prev_node);
                    prev.as_ref().unwrap().borrow_mut().right = None;
                    let cur_val = cur.as_ref().unwrap().borrow().val;
                    res.push(cur_val);
                    let right = cur.as_ref().unwrap().borrow().right.clone();
                    cur = right;
                }
            }
        }

        res
    }
}

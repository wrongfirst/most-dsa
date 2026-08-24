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
    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(node) => {
                let val = node.borrow().val;
                if key > val {
                    let right = node.borrow().right.clone();
                    let new_right = Self::delete_node(right, key);
                    node.borrow_mut().right = new_right;
                    Some(node)
                } else if key < val {
                    let left = node.borrow().left.clone();
                    let new_left = Self::delete_node(left, key);
                    node.borrow_mut().left = new_left;
                    Some(node)
                } else {
                    // Found the node to delete
                    let left = node.borrow().left.clone();
                    let right = node.borrow().right.clone();

                    if left.is_none() {
                        return right;
                    }
                    if right.is_none() {
                        return left;
                    }

                    // Node has two children - find inorder successor (leftmost in right subtree)
                    let right_ref = right.clone().unwrap();
                    let mut cur = right_ref.clone();

                    // Find the leftmost node in right subtree
                    while cur.borrow().left.is_some() {
                        let next = cur.borrow().left.clone().unwrap();
                        cur = next;
                    }

                    // Attach the left subtree to the leftmost node
                    cur.borrow_mut().left = left;

                    // Return the right subtree as the new root
                    right
                }
            }
        }
    }
}

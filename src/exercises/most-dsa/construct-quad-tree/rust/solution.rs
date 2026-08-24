// Definition for a QuadTree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct Node {
//     pub val: bool,
//     pub is_leaf: bool,
//     pub top_left: Option<Rc<RefCell<Node>>>,
//     pub top_right: Option<Rc<RefCell<Node>>>,
//     pub bottom_left: Option<Rc<RefCell<Node>>>,
//     pub bottom_right: Option<Rc<RefCell<Node>>>,
// }
//
// impl Node {
//     #[inline]
//     pub fn new(val: bool, is_leaf: bool) -> Self {
//         Node {
//             val,
//             is_leaf,
//             top_left: None,
//             top_right: None,
//             bottom_left: None,
//             bottom_right: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn construct(grid: Vec<Vec<i32>>) -> Option<Rc<RefCell<Node>>> {
        Self::dfs(&grid, grid.len(), 0, 0)
    }

    fn dfs(grid: &Vec<Vec<i32>>, n: usize, r: usize, c: usize) -> Option<Rc<RefCell<Node>>> {
        if n == 1 {
            return Some(Rc::new(RefCell::new(Node::new(grid[r][c] == 1, true))));
        }

        let mid = n / 2;
        let top_left = Self::dfs(grid, mid, r, c);
        let top_right = Self::dfs(grid, mid, r, c + mid);
        let bottom_left = Self::dfs(grid, mid, r + mid, c);
        let bottom_right = Self::dfs(grid, mid, r + mid, c + mid);

        let tl = top_left.as_ref().unwrap().borrow();
        let tr = top_right.as_ref().unwrap().borrow();
        let bl = bottom_left.as_ref().unwrap().borrow();
        let br = bottom_right.as_ref().unwrap().borrow();

        if tl.is_leaf && tr.is_leaf && bl.is_leaf && br.is_leaf
            && tl.val == tr.val && tl.val == bl.val && tl.val == br.val
        {
            return Some(Rc::new(RefCell::new(Node::new(tl.val, true))));
        }

        drop(tl);
        drop(tr);
        drop(bl);
        drop(br);

        let mut node = Node::new(false, false);
        node.top_left = top_left;
        node.top_right = top_right;
        node.bottom_left = bottom_left;
        node.bottom_right = bottom_right;
        Some(Rc::new(RefCell::new(node)))
    }
}

//Given a complete binary tree, count the number of nodes.
//
//Note:
//
//Definition of a complete binary tree from Wikipedia:
//In a complete binary tree every level, except possibly the last, is completely filled, and all nodes in the last level are as far left as possible. It can have between 1 and 2h nodes inclusive at the last level h.
//
//Example:
//
//Input: 
//    1
//   / \
//  2   3
// / \  /
//4  5 6
//
//Output: 6

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        /*
        match &root {
            Some(v) => 1 + Solution::count_nodes(v.borrow().left.clone()) + Solution::count_nodes(v.borrow().right.clone()),
            None => 0
        }
        */
        if let Some(v) = root {
            let mut l = 1;
            let mut r = 1;
            let mut node = v.borrow().left.clone();
            while let Some(n) = node {
                l += 1;
                node = n.borrow().left.clone();
            }
            node = v.borrow().right.clone();
            while let Some(n) = node {
                r += 1;
                node = n.borrow().right.clone();
            }
            if l == r {
                2i32.pow(l) - 1
            } else {
                1 + Solution::count_nodes(v.borrow().left.clone()) + Solution::count_nodes(v.borrow().right.clone())
            }
        } else {
            0
        }
    }
}
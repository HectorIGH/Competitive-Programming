//Find the sum of all left leaves in a given binary tree.
//
//Example:
//
//    3
//   / \
//  9  20
//    /  \
//   15   7
//
//There are two left leaves in the binary tree, with values 9 and 15 respectively. Return 24.

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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::aux(&root, 0)
    }
    
    pub fn aux(root: &Option<Rc<RefCell<TreeNode>>>, l: i32) -> i32 {
        if let Some(root) = root {
            if root.borrow().left.is_none() && root.borrow().right.is_none() {
                root.borrow().val * l
            } else {
                Solution::aux(&root.borrow().left, 1) + Solution::aux(&root.borrow().right, 0)
            }
        } else {
            0
        }
    }
}
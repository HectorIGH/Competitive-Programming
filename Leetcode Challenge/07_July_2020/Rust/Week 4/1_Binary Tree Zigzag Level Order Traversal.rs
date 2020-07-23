//Given a binary tree, return the zigzag level order traversal of its nodes' values. (ie, from left to right, then right to left for the next level and alternate between).
//
//For example:
//Given binary tree [3,9,20,null,null,15,7],
//    3
//   / \
//  9  20
//    /  \
//   15   7
//return its zigzag level order traversal as:
//[
//  [3],
//  [20,9],
//  [15,7]
//]

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
use std::collections::VecDeque;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut ans : Vec<Vec<i32>> = Vec::new();
        let mut q : VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        q.push_back(root.unwrap());
        let mut dir = false;
        while !q.is_empty() {
            let mut level : Vec<i32> = Vec::new();
            let mut tam = q.len();
            for _ in 0..tam {
                if let Some(node) = q.pop_front() {
                    level.push(node.borrow().val.clone());
                    if let Some(left) = node.borrow().left.clone() {
                        q.push_back(left);
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        q.push_back(right);
                    }
                }
            }
            if dir {
                level.reverse();
            }
            ans.push(level);
            dir = !dir;
        }
        return ans;
    }
}
//Given a binary tree, return the bottom-up level order traversal of its nodes' values. (ie, from left to right, level by level from leaf to root).
//
//For example:
//Given binary tree [3,9,20,null,null,15,7],
//    3
//   / \
//  9  20
//    /  \
//   15   7
//return its bottom-up level order traversal as:
//[
//  [15,7],
//  [9,20],
//  [3]
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
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        
        // BFS
        if root.is_none() {
            return Vec::new();
        }
        let mut ans : Vec<Vec<i32>> = Vec::new();
        let mut dq = VecDeque::new();
        dq.push_back(root.unwrap());
        while !dq.is_empty() {
            let mut holder : Vec<i32> = Vec::new();
            let n = dq.len();
            for i in 0..n {
                if let Some(node) = dq.pop_front() {
                    holder.push(node.borrow().val);
                    if let Some(left) = node.borrow().left.clone() {
                        dq.push_back(left);
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        dq.push_back(right);
                    }
                }
            }
            ans.push(holder);
        }
        ans.reverse();
        ans
        /*
        // DFS
        let mut ans : Vec<Vec<i32>> = Vec::new();
        Solution::dfs(&root, 0, &mut ans);
        ans.reverse();
        ans
        */
    }
    /*
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, level : usize, ans: &mut Vec<Vec<i32>>) {
        if let Some(node) = root {
            if level == ans.len() {
                ans.push(Vec::new());
            }
            ans[level].push(node.borrow().val.clone());
            Solution::dfs(&node.borrow().left, level + 1, ans);
            Solution::dfs(&node.borrow().right, level + 1, ans);
        }
    }
    */
}
//Given a binary tree, each node has value 0 or 1.  Each root-to-leaf path represents a binary number starting with the most significant bit.  For example, if the path is 0 -> 1 -> 1 -> 0 -> 1, then this could represent 01101 in binary, which is 13.
//
//For all leaves in the tree, consider the numbers represented by the path from the root to that leaf.
//
//Return the sum of these numbers.
//
// 
//
//Example 1:
//
//
//
//Input: [1,0,1,0,1,0,1]
//Output: 22
//Explanation: (100) + (101) + (110) + (111) = 4 + 5 + 6 + 7 = 22
// 
//
//Note:
//
//The number of nodes in the tree is between 1 and 1000.
//node.val is 0 or 1.
//The answer will not exceed 2^31 - 1.
//   Hide Hint #1  
//Find each path, then transform that path to an integer in base 10.


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
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut l: i32 = 0;
        Solution::dfs(&root, 0, &mut l);
        l
    }
    
    pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, s: i32, l: &mut i32) {
        if let Some(root) = root {
            if root.borrow().left.is_none() && root.borrow().right.is_none() {
                *l += s << 1 | root.borrow().val;
            }
            if root.borrow().left.is_some() {
                Solution::dfs(&root.borrow().left, s << 1 | root.borrow().val, l);
            }
            if root.borrow().right.is_some() {
                Solution::dfs(&root.borrow().right, s << 1 | root.borrow().val, l);
            }
        }
    }
}
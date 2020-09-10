//Given two binary search trees root1 and root2.
//
//Return a list containing all the integers from both trees sorted in ascending order.
//
// 
//
//Example 1:
//
//
//Input: root1 = [2,1,4], root2 = [1,0,3]
//Output: [0,1,1,2,3,4]
//Example 2:
//
//Input: root1 = [0,-10,10], root2 = [5,1,7,0,2]
//Output: [-10,0,0,1,2,5,7,10]
//Example 3:
//
//Input: root1 = [], root2 = [5,1,7,0,2]
//Output: [0,1,2,5,7]
//Example 4:
//
//Input: root1 = [0,-10,10], root2 = []
//Output: [-10,0,10]
//Example 5:
//
//
//Input: root1 = [1,null,8], root2 = [8,1]
//Output: [1,1,8,8]
// 
//
//Constraints:
//
//Each tree has at most 5000 nodes.
//Each node's value is between [-10^5, 10^5].
//   Hide Hint #1  
//Traverse the first tree in list1 and the second tree in list2.
//   Hide Hint #2  
//Merge the two trees in one list and sort it.

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
    pub fn get_all_elements(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut l1: Vec<i32> = Vec::new();
        let mut l2: Vec<i32> = Vec::new();
        Solution::dfs(&root1, &mut l1);
        Solution::dfs(&root2, &mut l2);
        l1.append(&mut l2);
        l1.sort_unstable();
        return l1;
    }
    
    pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, l: &mut Vec<i32>) {
        if let Some(root) = root {
            l.push(root.borrow().val);
            if root.borrow().left.is_some() {
                Solution::dfs(&root.borrow().left, l);
            }
            if root.borrow().right.is_some() {
                Solution::dfs(&root.borrow().right, l);
            }
        } else {
            return ;
        }
    }
}
//Given a binary tree, write a function to get the maximum width of the given tree. The maximum width of a tree is the maximum width among all levels.
//
//The width of one level is defined as the length between the end-nodes (the leftmost and right most non-null nodes in the level, where the null nodes between the end-nodes are also counted into the length calculation.
//
//It is guaranteed that the answer will in the range of 32-bit signed integer.
//
//Example 1:
//
//Input: 
//
//           1
//         /   \
//        3     2
//       / \     \  
//      5   3     9 
//
//Output: 4
//Explanation: The maximum width existing in the third level with the length 4 (5,3,null,9).
//Example 2:
//
//Input: 
//
//          1
//         /  
//        3    
//       / \       
//      5   3     
//
//Output: 2
//Explanation: The maximum width existing in the third level with the length 2 (5,3).
//Example 3:
//
//Input: 
//
//          1
//         / \
//        3   2 
//       /        
//      5      
//
//Output: 2
//Explanation: The maximum width existing in the second level with the length 2 (3,2).
//Example 4:
//
//Input: 
//
//          1
//         / \
//        3   2
//       /     \  
//      5       9 
//     /         \
//    6           7
//Output: 8
//Explanation:The maximum width existing in the fourth level with the length 8 (6,null,null,null,null,null,null,7).
// 
//
//Constraints:
//
//The given binary tree will have between 1 and 3000 nodes.

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
use std::cmp::max;
impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        /*
        // BFS
        let mut width = 0;
        let mut n = 0;
        let mut q : VecDeque<(Rc<RefCell<TreeNode>>, i32)> = VecDeque::new();
        q.push_back((root.unwrap(), 1));
        while !q.is_empty() {
            n = q.len();
            width = max(width, q.back().expect("no back").1 - q.front().expect("no front").1 + 1);
            while n != 0 {
                if let Some(node) = q.pop_front() {
                    if let Some(left) = node.0.borrow().left.clone() {
                        q.push_back((left, 2 * node.1));
                    }
                    if let Some(right) = node.0.borrow().right.clone() {
                        q.push_back((right, 2 * node.1 + 1));
                    }
                }
                n -= 1;
            }
        }
        return width;
        */
        
        // DFS
        let mut levelLMN : Vec<i32> = Vec::new();
        return Solution::dfs(&root, 1, 0, &mut levelLMN);
        
    }
    
    fn dfs(root : &Option<Rc<RefCell<TreeNode>>>, id : i32, level : i64, levelLMN : &mut Vec<i32>) -> i32 {
        if let Some(node) = root {
            if level as usize == levelLMN.len() {
                levelLMN.push(id);
            }
            return max(id + 1 - levelLMN[level as usize],
                max(Solution::dfs(&node.borrow().left, 2 * id, level + 1, levelLMN), Solution::dfs(&node.borrow().right, 2 * id + 1, level + 1, levelLMN)));
        }
        0
    }
    
}
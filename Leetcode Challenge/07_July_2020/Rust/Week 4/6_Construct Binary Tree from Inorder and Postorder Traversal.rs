//Given inorder and postorder traversal of a tree, construct the binary tree.
//
//Note:
//You may assume that duplicates do not exist in the tree.
//
//For example, given
//
//inorder = [9,3,15,20,7]
//postorder = [9,15,7,20,3]
//Return the following binary tree:
//
//    3
//   / \
//  9  20
//    /  \
//   15   7

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
use std::collections::HashMap;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut indexes : HashMap<i32, i32> = HashMap::new();
        for i in 0..inorder.len() {
            indexes.insert(inorder[i], i as i32);
        }
        Solution::aux(&inorder, &postorder, 0, inorder.len() as i32 - 1, postorder.len() - 1, &indexes)
    }
    pub fn aux(inorder : &Vec<i32>, postorder : &Vec<i32>, start : i32, end : i32, index : usize, indexes : &HashMap<i32, i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if start > end {
            return None;
        }
        Some(Rc::new(RefCell::new(TreeNode {val : postorder[index],
        right : Solution::aux(inorder, postorder, indexes[&postorder[index]] + 1, end, index - 1, indexes),
        left : Solution::aux(inorder, postorder, start, indexes[&postorder[index]] - 1, index - (end - indexes[&postorder[index]]) as usize - 1, indexes)})))
    }
}
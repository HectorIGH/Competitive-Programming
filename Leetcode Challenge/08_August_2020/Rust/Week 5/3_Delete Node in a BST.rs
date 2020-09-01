//Given a root node reference of a BST and a key, delete the node with the given key in the BST. Return the root node reference (possibly updated) of the BST.
//
//Basically, the deletion can be divided into two stages:
//
//Search for a node to remove.
//If the node is found, delete the node.
//Note: Time complexity should be O(height of tree).
//
//Example:
//
//root = [5,3,6,2,4,null,7]
//key = 3
//
//    5
//   / \
//  3   6
// / \   \
//2   4   7
//
//Given key to delete is 3. So we find the node with value 3 and delete it.
//
//One valid answer is [5,4,6,2,null,null,7], shown in the following BST.
//
//    5
//   / \
//  4   6
// /     \
//2       7
//
//Another valid answer is [5,2,6,null,4,null,7].
//
//    5
//   / \
//  2   6
//   \   \
//    4   7

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
    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::helper(&root, key)
    }
    
    pub fn helper(root: &Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root {
            if root.borrow().val == key {
                if root.borrow().right.is_none() {
                    return root.borrow().left.clone();
                }
                if root.borrow().left.is_none() {
                    return root.borrow().right.clone();
                }
                if root.borrow().left.is_some() && root.borrow().right.is_some() {
                    let temp = Solution::search_last_left(&root.borrow().right);
                    if let Some(new_val) = temp {
                        root.borrow_mut().val = new_val;
                        let r = Solution::helper(&root.borrow().right, new_val);
                        root.borrow_mut().right = r;
                    }
                }
            } else if root.borrow().val > key {
                let l = Solution::helper(&root.borrow().left, key);
                root.borrow_mut().left = l;
            } else {
                let r = Solution::helper(&root.borrow().right, key);
                root.borrow_mut().right = r;
            }
        }
        root.clone()
    }
    pub fn search_last_left(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        if let Some(node) = root {
            if node.borrow().left.is_some() {
                Solution::search_last_left(&node.borrow().left)
            } else {
                Some(node.borrow().val)
            }
        } else {
            None
        }
    }
}
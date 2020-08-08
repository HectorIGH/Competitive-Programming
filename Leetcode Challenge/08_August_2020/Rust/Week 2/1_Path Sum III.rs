//You are given a binary tree in which each node contains an integer value.
//
//Find the number of paths that sum to a given value.
//
//The path does not need to start or end at the root or a leaf, but it must go downwards (traveling only from parent nodes to child nodes).
//
//The tree has no more than 1,000 nodes and the values are in the range -1,000,000 to 1,000,000.
//
//Example:
//
//root = [10,5,-3,3,2,null,11,3,-2,null,1], sum = 8
//
//      10
//     /  \
//    5   -3
//   / \    \
//  3   2   11
// / \   \
//3  -2   1
//
//Return 3. The paths that sum to 8 are:
//
//1.  5 -> 3
//2.  5 -> 2 -> 1
//3. -3 -> 11

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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        /*
        if let Some(root) = root {
            return Solution::dfs(Some(root.clone()), sum) + Solution::path_sum(root.borrow().left.clone(), sum) + Solution::path_sum(root.borrow().right.clone(), sum);
        } else {
            return 0;
        }
        */
        let mut result: i32 = 0;
        let mut count: HashMap<i32, i32> = HashMap::new();
        count.insert(sum, 1);
        Solution::dfs(root, sum , 0, &mut result, &mut count);
        return result;
    }
    /*
    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32{
        if let Some(root) = root {
            let mut ans: i32 = if root.borrow().val.clone() == sum {1} else {0};
            return ans + Solution::dfs(root.borrow().left.clone(), sum - root.borrow().val.clone()) + Solution::dfs(root.borrow().right.clone(), sum - root.borrow().val.clone());
        } else {
            return 0;
        }
    }
    */
    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, sum: i32, root_sum: i32, result: &mut i32, count: &mut HashMap<i32, i32>) {
        if let Some(root) = root {
            let mut root_sum: i32 = root_sum;
            root_sum += root.borrow().val.clone();
            
            *result += *count.entry(root_sum).or_insert(0);
            
            let mut cur: i32 = *count.entry(root_sum + sum).or_insert(0);
            count.insert(root_sum + sum, cur + 1);
            
            Solution::dfs(root.borrow().left.clone(), sum, root_sum, result, count);
            Solution::dfs(root.borrow().right.clone(), sum, root_sum, result, count);

            cur = *count.entry(root_sum + sum).or_default();
            count.insert(root_sum + sum, cur - 1);
        } else {
            ()
        }
    }
}
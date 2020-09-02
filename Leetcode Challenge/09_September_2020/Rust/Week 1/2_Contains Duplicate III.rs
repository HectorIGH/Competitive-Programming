//Given an array of integers, find out whether there are two distinct indices i and j in the array such that the absolute difference between nums[i] and nums[j] is at most t and the absolute difference between i and j is at most k.
//
//Example 1:
//
//Input: nums = [1,2,3,1], k = 3, t = 0
//Output: true
//Example 2:
//
//Input: nums = [1,0,1,1], k = 1, t = 2
//Output: true
//Example 3:
//
//Input: nums = [1,5,9,1,5,9], k = 2, t = 3
//Output: false
//   Hide Hint #1  
//Time complexity O(n logk) - This will give an indication that sorting is involved for k elements.
//   Hide Hint #2  
//Use already existing state to evaluate next state - Like, a set of k sorted numbers are only needed to be tracked. When we are processing the next number in array, then we can utilize the existing sorted state and it is not necessary to sort next overlapping set of k numbers again.

use std::collections::BTreeSet;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        if t < 0 {
            return false;
        }
        let k: usize = k as usize;
        let t: i64 = t as i64;
        let nums: Vec<i64> = nums.iter().map(|x| *x as i64).collect();
        let mut tree: BTreeSet<i64> = BTreeSet::new();
        let mut n: i64 = 0;
        for i in 0..nums.len() {
            n = nums[i];
            if tree.range((n - t)..=(n + t)).count() > 0 {
                return true;
            }
            tree.insert(n);
            if i >= k {
                tree.remove(&nums[i - k]);
            }
        }
        false
    }
}
//Given a non-empty array of integers, return the k most frequent elements.
//
//Example 1:
//
//Input: nums = [1,1,1,2,2,3], k = 2
//Output: [1,2]
//Example 2:
//
//Input: nums = [1], k = 1
//Output: [1]
//Note:
//
//You may assume k is always valid, 1 ≤ k ≤ number of unique elements.
//Your algorithm's time complexity must be better than O(n log n), where n is the array's size.
//It's guaranteed that the answer is unique, in other words the set of the top k frequent elements is unique.
//You can return the answer in any order.

use std::collections::{HashMap, BinaryHeap};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.len() == k as usize {
            return nums;
        }
        let mut ans : Vec<i32> = vec![0; k as usize];
        let mut freq = HashMap::new();
        let mut heap : BinaryHeap<(i32, i32)> = BinaryHeap::new();
        for n in &nums {
            *freq.entry(n).or_insert(0) += 1;
        }
        for (number, frequency) in freq {
            heap.push((frequency, *number));
        }
        for i in 0..k as usize {
            ans[i] = heap.pop().unwrap().1;
        }
        return ans;
    }
}
//Given a sorted (in ascending order) integer array nums of n elements and a target value, write a function to search target in nums. If target exists, then return its index, otherwise return -1.
//
//
//Example 1:
//
//Input: nums = [-1,0,3,5,9,12], target = 9
//Output: 4
//Explanation: 9 exists in nums and its index is 4
//
//Example 2:
//
//Input: nums = [-1,0,3,5,9,12], target = 2
//Output: -1
//Explanation: 2 does not exist in nums so return -1
// 
//
//Note:
//
//You may assume that all elements in nums are unique.
//n will be in the range [1, 10000].
//The value of each element in nums will be in the range [-9999, 9999].

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut lo: i32 = 0;
        let mut hi: i32 = nums.len() as i32 - 1;
        let mut mi: i32 = 0;
        while lo <= hi {
            mi = lo + (hi - lo) / 2;
            if nums[mi as usize] == target {
                return mi;
            } else if nums[mi as usize] < target {
                lo = mi + 1;
            } else {
                hi = mi - 1;
            }
        }
        -1
    }
}
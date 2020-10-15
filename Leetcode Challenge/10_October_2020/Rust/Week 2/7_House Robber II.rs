//You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed. All houses at this place are arranged in a circle. That means the first house is the neighbor of the last one. Meanwhile, adjacent houses have a security system connected, and it will automatically contact the police if two adjacent houses were broken into on the same night.
//
//Given a list of non-negative integers nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.
//
// 
//
//Example 1:
//
//Input: nums = [2,3,2]
//Output: 3
//Explanation: You cannot rob house 1 (money = 2) and then rob house 3 (money = 2), because they are adjacent houses.
//Example 2:
//
//Input: nums = [1,2,3,1]
//Output: 4
//Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
//Total amount you can rob = 1 + 3 = 4.
//Example 3:
//
//Input: nums = [0]
//Output: 0
// 
//
//Constraints:
//
//1 <= nums.length <= 100
//0 <= nums[i] <= 1000
//   Hide Hint #1  
//Since House[1] and House[n] are adjacent, they cannot be robbed together. Therefore, the problem becomes to rob either House[1]-House[n-1] or House[2]-House[n], depending on which choice offers more money. Now the problem has degenerated to the House Robber, which is already been solved.

use std::cmp::max;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        //return max(nums[0] + Solution::rob_help(&nums, 2, nums.len() - 1), Solution::rob_help(&nums, 1, nums.len()));
        if nums.len() <= 2 {
            return *nums.iter().max().unwrap();
        }
        return max(nums[0] + Solution::rob_help(&nums[2..nums.len() - 1]), Solution::rob_help(&nums[1..]));
    }
    
    //pub fn rob_help(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
    pub fn rob_help(nums: &[i32]) -> i32 {
        let mut dp1: i32 = 0;
        let mut dp2: i32 = 0;
        let mut num: i32 = 0;
        let mut temp: i32 = 0;
        //for i in start..end {
        for num in nums {
            temp = dp2;
            //dp2 = max(dp1 + nums[i], dp2);
            dp2 = max(dp1 + num, dp2);
            dp1 = temp;
        }
        return dp2;
    }
}
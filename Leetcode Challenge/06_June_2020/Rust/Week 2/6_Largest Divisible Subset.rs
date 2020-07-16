//Given a set of distinct positive integers, find the largest subset such that every pair (Si, Sj) of elements in this subset satisfies:
//
//Si % Sj = 0 or Sj % Si = 0.
//
//If there are multiple solutions, return any subset is fine.
//
//Example 1:
//
//Input: [1,2,3]
//Output: [1,2] (of course, [1,3] will also be ok)
//Example 2:
//
//Input: [1,2,4,8]
//Output: [1,2,4,8]

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n <= 1 {
            return nums;
        }
        let mut nums = nums;
        nums.sort();
        let mut divCount = vec![1; n];
        let mut prev = vec![-1 as i32; n];
        let mut ans = vec![];
        let mut max_index:i32 = 0;
        for i in 1..n {
            for j in 0..i {
                if nums[i] % nums[j] == 0 {
                    if divCount[i] <= divCount[j] {
                        divCount[i] = divCount[j] + 1;
                        prev[i] = j as i32;
                    }
                }
            }
            if divCount[max_index as usize] < divCount[i] {
                max_index = i as i32;
            }
        }
        while max_index >= 0 {
            ans.push(nums[max_index as usize]);
            max_index = prev[max_index as usize];
        }
        return ans;
    }
}
//Given an array nums of n integers, are there elements a, b, c in nums such that a + b + c = 0? Find all unique triplets in the array which gives the sum of zero.
//
//Note:
//
//The solution set must not contain duplicate triplets.
//
//Example:
//
//Given array nums = [-1, 0, 1, 2, -1, -4],
//
//A solution set is:
//[
//  [-1, 0, 1],
//  [-1, -1, 2]
//]
//   Hide Hint #1  
//So, we essentially need to find three numbers x, y, and z such that they add up to the given value. If we fix one of the numbers say x, we are left with the two-sum problem at hand!
//   Hide Hint #2  
//For the two-sum problem, if we fix one of the numbers, say
//x
//, we have to scan the entire array to find the next number
//y
//which is
//value - x
//where value is the input parameter. Can we change our array somehow so that this search becomes faster?
//   Hide Hint #3  
//The second train of thought for two-sum is, without changing the array, can we use additional space somehow? Like maybe a hash map to speed up the search?

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let n = nums.len();
        let mut ans : Vec<Vec<i32>> = Vec::new();
        nums.sort_unstable();
        if n < 3 || nums[0] > 0 {
            return ans;
        }
        let mut first = 0;
        let mut last = 0;
        let mut sum = 0;
        for i in 0..(n - 2) {
            if i == 0 || nums[i] != nums[i - 1] {
                first = i + 1;
                last = n - 1;
                while first < last {
                    sum = nums[i] + nums[first] + nums[last];
                    if sum > 0 {
                        last -= 1;
                    } else if sum < 0 {
                        first += 1;
                    } else {
                        ans.push(vec![nums[i], nums[first], nums[last]]);
                        while first < last && nums[first] == nums[first + 1] {
                            first += 1;
                        }
                        while first < last && nums[last] == nums[last - 1] {
                            last -= 1;
                        }
                        first += 1;
                        last -= 1;
                    }
                }
            }
        }
        ans
    }
}
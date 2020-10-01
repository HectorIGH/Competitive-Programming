//Given an unsorted integer array, find the smallest missing positive integer.
//
//Example 1:
//
//Input: [1,2,0]
//Output: 3
//Example 2:
//
//Input: [3,4,-1,1]
//Output: 2
//Example 3:
//
//Input: [7,8,9,11,12]
//Output: 1
//Follow up:
//
//Your algorithm should run in O(n) time and uses constant extra space.
//
//   Hide Hint #1  
//Think about how you would solve the problem in non-constant space. Can you apply that logic to the existing space?
//   Hide Hint #2  
//We don't care about duplicates or non-positive integers
//   Hide Hint #3  
//Remember that O(2n) = O(n)

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        /*
        unordered_set<int> numS(nums.begin(), nums.end());
        int i = 1;
        while(true) {
            if(!numS.count(i)) {
                return i;
            }
            i += 1;
        }
        */
        
        if nums.len() == 0 {
            return 1;
        }
        let n: usize = nums.len();
        let mut containsone: bool = false;
        for i in 0..n {
            if nums[i] == 1 {
                containsone = true;
                break;
            }
        }
        if !containsone {
            return 1;
        }
        for i in 0..n {
            if nums[i] <= 0 || nums[i] > n as i32 {
                nums[i] = 1;
            }
        }
        let mut val: i32 = 0;
        let mut pos: usize = 0;
        for i in 0..n {
            val = nums[i];
            pos = val.abs() as usize - 1;
            if nums[pos] > 0 {
                nums[pos] = -1 * nums[pos];
            }
        }
        for i in 0..n {
            if nums[i] > 0 {
                return i as i32 + 1;
            }
        }
        return n as i32 + 1;
        
    }
}
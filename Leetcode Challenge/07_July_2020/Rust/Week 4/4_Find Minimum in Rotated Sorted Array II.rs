//Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
//
//(i.e.,  [0,1,2,4,5,6,7] might become  [4,5,6,7,0,1,2]).
//
//Find the minimum element.
//
//The array may contain duplicates.
//
//Example 1:
//
//Input: [1,3,5]
//Output: 1
//Example 2:
//
//Input: [2,2,2,0,1]
//Output: 0
//Note:
//
//This is a follow up problem to Find Minimum in Rotated Sorted Array.
//Would allow duplicates affect the run-time complexity? How and why?

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        
        let mut l : i32  = 0;
        let mut r = (nums.len() - 1) as i32;
        let mut m : usize = 0;
        while l < r {
            m = (l + ((r - l) >> 1)) as usize;
            if nums[m] > nums[r as usize] {
                l = m as i32 + 1;
            } else if nums[m] < nums[r as usize] {
                r = m as i32;
            } else {
                r -= 1;
            }
        }
        nums[l as usize]
        
        //Solution::aux(0, nums.len() as i32 - 1, nums)
    }
    /*
    pub fn aux(l : i32, r : i32, nums : Vec<i32>) -> i32 {
        if l > r {
            nums[l as usize]
        } else {
            let mut m = (l + ((r - l) >> 1)) as usize;
            if nums[m] > nums[r as usize] {
                Solution::aux(m as i32 + 1, r, nums)
            } else if nums[m] < nums[r as usize] {
                Solution::aux(l, m as i32, nums)
            } else {
                Solution::aux(l, r - 1, nums)
            }
        }
    }
    */
}
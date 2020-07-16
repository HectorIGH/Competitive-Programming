//Given a sorted array and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
//
//You may assume no duplicates in the array.
//
//Example 1:
//
//Input: [1,3,5,6], 5
//Output: 2
//Example 2:
//
//Input: [1,3,5,6], 2
//Output: 1
//Example 3:
//
//Input: [1,3,5,6], 7
//Output: 4
//Example 4:
//
//Input: [1,3,5,6], 0
//Output: 0

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l:i32 = 0;
        let mut r:i32 = (nums.len() - 1) as i32;
        let mut mid:i32 = 0;
        while l<= r {
            mid = l + (r - l) / 2;
            if nums[mid as usize] == target {
                return mid;
            }
            if nums[mid as usize] < target {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        return l;
        /*
        match nums.binary_search(&target) {
            Ok(index) => index as i32,
            Err(index) => index as i32
        }
        */
    }
}
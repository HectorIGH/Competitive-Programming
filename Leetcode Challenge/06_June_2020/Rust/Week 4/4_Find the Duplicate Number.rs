//Given an array nums containing n + 1 integers where each integer is between 1 and n (inclusive), prove that at least one duplicate number must exist. Assume that there is only one duplicate number, find the duplicate one.
//
//Example 1:
//
//Input: [1,3,4,2,2]
//Output: 2
//Example 2:
//
//Input: [3,1,3,4,2]
//Output: 3
//Note:
//
//You must not modify the array (assume the array is read only).
//You must use only constant, O(1) extra space.
//Your runtime complexity should be less than O(n2).
//There is only one duplicate number in the array, but it could be repeated more than once.

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = nums[0] as usize;
        let mut fast = nums[0] as usize;
        loop {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;
            if slow == fast {
                break;
            }
        }
        slow = nums[0] as usize;
        while slow != fast {
            slow = nums[slow] as usize;
            fast = nums[fast] as usize;
        }
        return slow as i32;
        /*
        let mut l : i32 = 0;
        let mut r : i32 = (nums.len() - 1) as i32;
        let mut m : i32 = 0;
        let mut count : i32 = 0;
        while l < r {
            m = l + (r - l) / 2;
            count = 0;
            for n in &nums {
                count += if n <= &m {1} else {0};
            }
            if count <= m {
                l = m + 1;
            } else {
                r = m;
            }
        }
        return r;
        */
    }
}
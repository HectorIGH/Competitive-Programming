//Given an array with n objects colored red, white or blue, sort them in-place so that objects of the same color are adjacent, with the colors in the order red, white and blue.
//
//Here, we will use the integers 0, 1, and 2 to represent the color red, white, and blue respectively.
//
//Note: You are not suppose to use the library's sort function for this problem.
//
//Example:
//
//Input: [2,0,2,1,1,0]
//Output: [0,0,1,1,2,2]
//Follow up:
//
//A rather straight forward solution is a two-pass algorithm using counting sort.
//First, iterate the array counting number of 0's, 1's, and 2's, then overwrite array with total number of 0's, then 1's and followed by 2's.
//Could you come up with a one-pass algorithm using only constant space?

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut l:usize = 0;
        let mut m:usize = 0;
        let mut r:i32 = (nums.len() - 1) as i32;
        let mut aux:i32 = 0;
        while m as i32 <= r {
            if nums[m] == 0 {
                aux = nums[m];
                nums[m] = nums[l];
                nums[l] = aux;
                m += 1;
                l += 1;
            } else if nums[m] == 1 {
                m += 1;
            } else {
                aux = nums[r as usize];
                nums[r as usize] = nums[m];
                nums[m] = aux;
                r -= 1;
            }
        }
    }
}
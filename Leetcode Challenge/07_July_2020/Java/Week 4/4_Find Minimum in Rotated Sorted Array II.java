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

class Solution {
    public int findMin(int[] nums) {
        
        int l = 0, r = nums.length - 1, m = 0;
        while(l < r) {
            m = l + ((r - l) >> 1);
            if(nums[m] > nums[r]) {
                l = m + 1;
            }else if(nums[m] < nums[r]) {
                r = m;
            } else {
                r--;
            }
        }
        return nums[l];
        
        //return aux(0, nums.length - 1, nums);
    }
    /*
    public int aux(int l, int r, int[] nums) {
        if(l > r) {
            return nums[l];
        }
        int m = l + ((r - l) >> 1);
        if(nums[m] > nums[r]) {
            return aux(m + 1, r, nums);
        } else if(nums[m] < nums[r]) {
            return aux(l, m, nums);
        } else {
            return aux(l, r - 1, nums);
        }
    }
    */
}
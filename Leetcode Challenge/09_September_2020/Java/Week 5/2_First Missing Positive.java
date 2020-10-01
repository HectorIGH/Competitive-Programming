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

class Solution {
    public int firstMissingPositive(int[] nums) {
        /*
        HashSet<Integer> numS = new HashSet<>();
        for(int n : nums) {
            if(n > 0) {
                numS.add(n);
            }
        }
        int i = 1;
        while(true) {
            if(!numS.contains(i)) {
                return i;
            }
            i += 1;
        }
        */
        
        if(nums.length == 0) {
            return 1;
        }
        int n = nums.length;
        boolean containsone = false;
        for(int i = 0; i < n; i++) {
            if(nums[i] == 1) {
                containsone = true;
                break;
            }
        }
        if(!containsone) {
            return 1;
        }
        for(int i = 0; i < n; i++) {
            if(nums[i] <= 0 || nums[i] > n) {
                nums[i] = 1;
            }
        }
        int val = 0;
        int pos = 0;
        for(int i = 0; i < n; i++) {
            val = nums[i];
            pos = Math.abs(val) - 1;
            if(nums[pos] > 0) {
                nums[pos] = -1 * nums[pos];
            }
        }
        for(int i = 0; i < n; i++) {
            if(nums[i] > 0) {
                return i + 1;
            }
        }
        return n + 1;
        
    }
}
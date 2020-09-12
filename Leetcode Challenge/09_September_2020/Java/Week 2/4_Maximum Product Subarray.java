//Given an integer array nums, find the contiguous subarray within an array (containing at least one number) which has the largest product.
//
//Example 1:
//
//Input: [2,3,-2,4]
//Output: 6
//Explanation: [2,3] has the largest product 6.
//Example 2:
//
//Input: [-2,0,-1]
//Output: 0
//Explanation: The result cannot be 2, because [-2,-1] is not a subarray.

class Solution {
    public int maxProduct(int[] nums) {
        int N = nums.length;
        ArrayList<Integer> dp1 = new ArrayList<>();
        ArrayList<Integer> dp2 = new ArrayList<>();
        dp1.add(nums[0]);
        dp2.add(nums[0]);
        for(int k = 1; k < N; k++) {
            if(nums[k] > 0) {
                dp1.add(Math.max(dp1.get(k-1) * nums[k], nums[k]));
                dp2.add(Math.min(dp2.get(k-1) * nums[k], nums[k]));
            } else {
                dp1.add(Math.max(dp2.get(k-1) * nums[k], nums[k]));
                dp2.add(Math.min(dp1.get(k-1) * nums[k], nums[k]));
            }
        }
        return Collections.max(dp1);
    }
}
//Your are given an array of positive integers nums.
//
//Count and print the number of (contiguous) subarrays where the product of all the elements in the subarray is less than k.
//
//Example 1:
//Input: nums = [10, 5, 2, 6], k = 100
//Output: 8
//Explanation: The 8 subarrays that have product less than 100 are: [10], [5], [2], [6], [10, 5], [5, 2], [2, 6], [5, 2, 6].
//Note that [10, 5, 2] is not included as the product of 100 is not strictly less than k.
//Note:
//
//0 < nums.length <= 50000.
//0 < nums[i] < 1000.
//0 <= k < 10^6.
//   Hide Hint #1  
//For each j, let opt(j) be the smallest i so that nums[i] * nums[i+1] * ... * nums[j] is less than k. opt is an increasing function.

class Solution {
    public int numSubarrayProductLessThanK(int[] nums, int k) {
        
        if(k <= 1) {
            return 0;
        }
        int prod = 1;
        int ans = 0;
        int left = 0;
        int val = 0;
        for(int right = 0; right < nums.length; right++) {
            val = nums[right];
            prod *= val;
            while(prod >= k && left <= right) {
                prod /= nums[left];
                left += 1;
            }
            ans += right - left + 1;
        }
        return ans;
        
        /*
        if(k == 0) {
            return 0;
        }
        double lk = Math.log(k);
        double[] prefix = new double[nums.length + 1];
        for(int i = 0; i < nums.length; i++) {
            prefix[i + 1] = prefix[i] + Math.log(nums[i]);
        }
        int ans = 0;
        int m = 0;
        int lo = 0;
        int hi = 0;
        for(int i = 0; i < prefix.length; i++) {
            lo = i + 1;
            hi = prefix.length;
            while(lo < hi) {
                m = lo + (hi - lo) / 2;
                if(prefix[m] < prefix[i] + lk - 1e-9) {
                    lo = m + 1;
                } else {
                    hi = m;
                }
            }
            ans += lo - i -1;
        }
        return ans;
        */
    }
}
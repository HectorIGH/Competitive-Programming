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
public:
    int numSubarrayProductLessThanK(vector<int>& nums, int k) {
        
        if(k <= 1) {
            return 0;
        }
        int prod = 1;
        int ans = 0;
        int left = 0;
        int val = 0;
        for(int right = 0; right < nums.size(); right++) {
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
        if(k <= 0) {
            return 0;
        }
        int n = nums.size();
        double lk = log10(k);
        vector<double> prefix(n + 1, 0.0);
        for(int i = 0; i < n; i++) {
            prefix[i + 1] += prefix[i] + log10(nums[i]);
        }
        int ans = 0;
        double x = 0;
        int j = 0;
        for(int i = 0; i < n; i++) {
            x = prefix[i];
            j = lower_bound(prefix.begin() + i + 1, prefix.end(), x + lk - (1e-9)) - prefix.begin();
            ans += j - i - 1;
        }
        return ans;
        */
    }
};
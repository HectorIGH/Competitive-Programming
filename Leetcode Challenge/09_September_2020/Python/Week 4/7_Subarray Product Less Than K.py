#Your are given an array of positive integers nums.
#
#Count and print the number of (contiguous) subarrays where the product of all the elements in the subarray is less than k.
#
#Example 1:
#Input: nums = [10, 5, 2, 6], k = 100
#Output: 8
#Explanation: The 8 subarrays that have product less than 100 are: [10], [5], [2], [6], [10, 5], [5, 2], [2, 6], [5, 2, 6].
#Note that [10, 5, 2] is not included as the product of 100 is not strictly less than k.
#Note:
#
#0 < nums.length <= 50000.
#0 < nums[i] < 1000.
#0 <= k < 10^6.
#   Hide Hint #1  
#For each j, let opt(j) be the smallest i so that nums[i] * nums[i+1] * ... * nums[j] is less than k. opt is an increasing function.

class Solution:
    def numSubarrayProductLessThanK(self, nums: List[int], k: int) -> int:
        
        if k <= 1: 
            return 0
        prod = 1
        ans = 0
        left = 0
        for right, val in enumerate(nums):
            prod *= val
            while prod >= k and left <= right:
                prod /= nums[left]
                left += 1
            ans += right - left + 1
        return ans
        '''
        if k == 0:
            return 0
        k = math.log(k)
        prefix = [0]
        for n in nums:
            prefix.append(prefix[-1] + math.log(n))
        ans = 0
        for i, x in enumerate(prefix):
            j = bisect.bisect(prefix, x + k - 1e-9, i + 1)
            ans += j - i - 1
        return ans
        '''
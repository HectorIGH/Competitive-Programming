#Given an integer array nums, find the contiguous subarray within an array (containing at least one number) which has the largest product.
#
#Example 1:
#
#Input: [2,3,-2,4]
#Output: 6
#Explanation: [2,3] has the largest product 6.
#Example 2:
#
#Input: [-2,0,-1]
#Output: 0
#Explanation: The result cannot be 2, because [-2,-1] is not a subarray.

class Solution:
    def maxProduct(self, nums: List[int]) -> int:
        N = len(nums)
        dp1 = [0] * N
        dp2 = [0] * N
        dp1[0] = dp2[0] = nums[0]
        for k in range(1, N):
            if nums[k] > 0:
                dp1[k] = max(dp1[k - 1] * nums[k], nums[k])
                dp2[k] = min(dp2[k - 1] * nums[k], nums[k])
            else:
                dp1[k] = max(dp2[k - 1] * nums[k], nums[k])
                dp2[k] = min(dp1[k - 1] * nums[k], nums[k])
        return max(dp1)
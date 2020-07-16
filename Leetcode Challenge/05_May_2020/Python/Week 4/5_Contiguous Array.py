#Given a binary array, find the maximum length of a contiguous subarray with equal number of 0 and 1.
#
#Example 1:
#Input: [0,1]
#Output: 2
#Explanation: [0, 1] is the longest contiguous subarray with equal number of 0 and 1.
#Example 2:
#Input: [0,1,0]
#Output: 2
#Explanation: [0, 1] (or [1, 0]) is a longest contiguous subarray with equal number of 0 and 1.
#Note: The length of the given binary array will not exceed 50,000.

class Solution:
    def findMaxLength(self, nums: List[int]) -> int:
        
        nums = [nums[i] if nums[i] else -1 for i in range(len(nums))]
        max_len = 0
        current_sum = 0
        d = {0: -1}
        
        for i, e in enumerate(nums):
            current_sum += e
            if current_sum in d:
                max_len = max(max_len, i - d[current_sum])
            else:
                d[current_sum] = i
        return max_len
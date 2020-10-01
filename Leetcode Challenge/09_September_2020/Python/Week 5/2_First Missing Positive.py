#Given an unsorted integer array, find the smallest missing positive integer.
#
#Example 1:
#
#Input: [1,2,0]
#Output: 3
#Example 2:
#
#Input: [3,4,-1,1]
#Output: 2
#Example 3:
#
#Input: [7,8,9,11,12]
#Output: 1
#Follow up:
#
#Your algorithm should run in O(n) time and uses constant extra space.
#
#   Hide Hint #1  
#Think about how you would solve the problem in non-constant space. Can you apply that logic to the existing space?
#   Hide Hint #2  
#We don't care about duplicates or non-positive integers
#   Hide Hint #3  
#Remember that O(2n) = O(n)

class Solution:
    def firstMissingPositive(self, nums: List[int]) -> int:
        '''
        nums = set((n for n in nums if n > 0))
        i = 1
        while True:
            if i not in nums:
                return i
            i += 1
        '''
        
        if len(nums) == 0:
            return 1
        n = len(nums);
        containsone = False;
        for i in range(n):
            if nums[i] == 1:
                containsone = True
                break
        if not containsone:
            return 1
        for i in range(n):
            if nums[i] <= 0 or nums[i] > n:
                nums[i] = 1
        for i in range(n):
            val = nums[i]
            pos = abs(val) - 1
            if nums[pos] > 0:
                nums[pos] = -1 * nums[pos];
        for i in range(n):
            if nums[i] > 0:
                return i + 1
        return n + 1
        
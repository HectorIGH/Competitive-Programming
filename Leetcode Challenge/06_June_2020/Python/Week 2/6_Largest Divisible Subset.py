#Given a set of distinct positive integers, find the largest subset such that every pair (Si, Sj) of elements in this subset satisfies:
#
#Si % Sj = 0 or Sj % Si = 0.
#
#If there are multiple solutions, return any subset is fine.
#
#Example 1:
#
#Input: [1,2,3]
#Output: [1,2] (of course, [1,3] will also be ok)
#Example 2:
#
#Input: [1,2,4,8]
#Output: [1,2,4,8]

class Solution:
    def largestDivisibleSubset(self, nums: List[int]) -> List[int]:
        if not nums:
            return nums
        n = len(nums)
        nums.sort()
        divCount = [1] * n
        prev = [-1] * n
        max_index = 0
        ans = []
        
        for i in range(1, n):
            for j in range(i):
                if nums[i] % nums[j] == 0:
                    if divCount[i] <= divCount[j]:
                        divCount[i] = divCount[j] + 1
                        prev[i] = j
            if divCount[max_index] < divCount[i]:
                max_index = i

        while max_index >= 0:
            ans.append(nums[max_index])
            max_index = prev[max_index]
        return ans
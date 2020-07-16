#Given a set of distinct integers, nums, return all possible subsets (the power set).
#
#Note: The solution set must not contain duplicate subsets.
#
#Example:
#
#Input: nums = [1,2,3]
#Output:
#[
#  [3],
#  [1],
#  [2],
#  [1,2,3],
#  [1,3],
#  [2,3],
#  [1,2],
#  []
#]

class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        '''
        self.ans = []
        self.dfs([-1], nums)
        return self.ans
    
    def dfs(self, current, nums):
        self.ans.append([nums[i] for i in current][1:])
        for i in range(current[-1] + 1, len(nums)):
            self.dfs(current + [i], nums)
        '''
        '''
        ans = [[]]
        for num in nums:
            ans += [n + [num] for n in ans]
        return ans
        '''
        n = len(nums)
        ans = []
        for i in range(1 << n, 1 << (n + 1)):
            bm = bin(i)[3:] # bitmask
            ans.append([nums[j] for j in range(n) if bm[j] == '1'])
        return ans
        
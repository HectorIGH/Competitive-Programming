#Given a non-empty array of integers, every element appears three times except for one, which appears exactly once. Find that single one.
#
#Note:
#
#Your algorithm should have a linear runtime complexity. Could you implement it without using extra memory?
#
#Example 1:
#
#Input: [2,2,3,2]
#Output: 3
#Example 2:
#
#Input: [0,1,0,1,0,1,99]
#Output: 99

class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        # Bitwise O(1) space
        '''
        ones, twos, not_threes = 0, 0, 0
        for n in nums:
            twos |= ones & n
            ones ^= n
            not_threes = ~(ones & twos)
            ones &= not_threes
            twos &= not_threes
            
        return ones
        '''
        # HashMap space O(n)
        freq = {}
        for n in nums:
            freq[n] = freq.get(n, 0) + 1
        for n in set(nums):
            if freq[n] == 1:
                return n
#Given an array nums containing n + 1 integers where each integer is between 1 and n (inclusive), prove that at least one duplicate number must exist. Assume that there is only one duplicate number, find the duplicate one.
#
#Example 1:
#
#Input: [1,3,4,2,2]
#Output: 2
#Example 2:
#
#Input: [3,1,3,4,2]
#Output: 3
#Note:
#
#You must not modify the array (assume the array is read only).
#You must use only constant, O(1) extra space.
#Your runtime complexity should be less than O(n2).
#There is only one duplicate number in the array, but it could be repeated more than once.

class Solution:
    def findDuplicate(self, nums: List[int]) -> int:
        # Two pointer, Floyd's Tortoise and Hare
        slow, fast = nums[0], nums[0]
        while True:
            slow, fast = nums[slow], nums[nums[fast]]
            if slow == fast:
                break
        slow = nums[0]
        while slow != fast:
            slow, fast = nums[slow], nums[fast]
        return slow
        '''
        # Binary search
        l, r = 0, len(nums) - 1
        m = 0
        while l < r:
            m = l + (r - l) // 2
            count = 0
            for n in nums:
                count += n <= m
            if count <= m:
                l = m + 1
            else:
                r = m
        return r
        '''
#Given an array nums, write a function to move all 0's to the end of it while maintaining the relative order of the non-zero elements.
#
#Example:
#
#Input: [0,1,0,3,12]
#Output: [1,3,12,0,0]
#Note:
#
#1. You must do this in-place without making a copy of the array.
#2. Minimize the total number of operations.
def moveZeroes(self, nums: List[int]) -> None:
    """
    Do not return anything, modify nums in-place instead.
    """
    ind = 0
    for e in nums:
        if e:
            nums[ind] = e
            ind += 1
    
    for i in range(ind, len(nums)):
        nums[i] = 0
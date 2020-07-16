#Given an array of size n, find the majority element. The majority element is the element that appears more than ⌊ n/2 ⌋ times.
#
#You may assume that the array is non-empty and the majority element always exist in the array.
#
#Example 1:
#
#Input: [3,2,3]
#Output: 3
#Example 2:
#
#Input: [2,2,1,1,1,2,2]
#Output: 2

class Solution:
    def majorityElement(self, nums: List[int]) -> int:
        m = len(nums)
        nums.sort()
        return nums[m // 2]
        #for i in range(m//2 + 1):
            #if nums[i] == nums[i + m//2]:
                #return nums[i]
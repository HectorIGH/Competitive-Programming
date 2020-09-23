#Given an integer array of size n, find all elements that appear more than ⌊ n/3 ⌋ times.
#
#Note: The algorithm should run in linear time and in O(1) space.
#
#Example 1:
#
#Input: [3,2,3]
#Output: [3]
#Example 2:
#
#Input: [1,1,1,3,3,2,2,2]
#Output: [1,2]
#   Hide Hint #1  
#How many majority elements could it possibly have?
#Do you have a better hint? Suggest it!

class Solution:
    def majorityElement(self, nums: List[int]) -> List[int]:
        if not nums:
            return []
        count1 = 0
        count2 = 0
        candidate1 = None
        candidate2 = None
        result = []
        for n in nums:
            if candidate1 == n:
                count1 += 1
            elif candidate2 == n:
                count2 += 1
            elif count1 == 0:
                candidate1 = n
                count1 += 1
            elif count2 == 0:
                candidate2 = n
                count2 += 1
            else:
                count1 -= 1
                count2 -= 1
                
        for c in [candidate1, candidate2]:
            if nums.count(c) > len(nums) // 3:
                result.append(c)
                
        #count1 = 0
        #count2 = 0
        #for c in nums:
        #    if c == candidate1:
        #        count1 += 1
        #    if c == candidate2:
        #        count2 += 1
        #if count1 > len(nums) // 3:
        #    result.append(candidate1)
        #if count2 > len(nums) // 3:
        #    result.append(candidate2)
                
        return result
#You are given a sorted array consisting of only integers where every element appears exactly twice, except for one element which appears exactly once. Find this single element that appears only once.
#
#Follow up: Your solution should run in O(log n) time and O(1) space.
#
# 
#
#Example 1:
#
#Input: nums = [1,1,2,3,3,4,4,8,8]
#Output: 2
#Example 2:
#
#Input: nums = [3,3,7,7,10,11,11]
#Output: 10
# 
#
#Constraints:
#
#1 <= nums.length <= 10^5
#0 <= nums[i] <= 10^5

class Solution:
    def singleNonDuplicate(self, nums: List[int]) -> int:
        # O(N) forward solution
        #unique = 0
        #for i in nums:
            #unique ^= i
        #return unique
        # O(ln(N)) and O(1) solution
        def search(nums, l, r):
            if l > r:
                return None
            if l == r:
                return nums[l]
            mid = l + (r - l) // 2
            if mid & 1:
                if nums[mid] == nums[mid-1]:
                    return search(nums, mid+1, r)
                else:
                    return search(nums, l, mid - 1)
            else:
                if nums[mid] == nums[mid+1]:
                    return search(nums, mid + 2, r)
                else:
                    return search(nums, l, mid)
        if len(nums) == 1:
            return nums[0]
        return search(nums, 0, len(nums) - 1)
#Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
#
#(i.e., [0,1,2,4,5,6,7] might become [4,5,6,7,0,1,2]).
#
#You are given a target value to search. If found in the array return its index, otherwise return -1.
#
#You may assume no duplicate exists in the array.
#
#Your algorithm's runtime complexity must be in the order of O(log n).
#
#Example 1:
#
#Input: nums = [4,5,6,7,0,1,2], target = 0
#Output: 4
#Example 2:
#
#Input: nums = [4,5,6,7,0,1,2], target = 3
#Output: -1

class Solution:
    def search(self, nums: List[int], target: int) -> int:
        if not nums:
            return -1

        def helper(nums, target, start, end):
            if start == end:
                if nums[start] == target:
                    return start
                else:
                    return -1
            else:
                n = end - start + 1

                #start_left = start
                #end_left = (n // 2) + start + (n&1)
                end_left = (n >> 1) + start + (n&1)

                #start_right = end_left
                #end_right = end

                ans_l = helper(nums, target, start, end_left - 1)

                ans_r = helper(nums, target, end_left, end)

            return max(ans_l, ans_r)

        return helper(nums, target, 0, len(nums) - 1)

print(search([4,5,6,7,0,1,2], 0))
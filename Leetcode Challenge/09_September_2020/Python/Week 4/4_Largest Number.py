#Given a list of non negative integers, arrange them such that they form the largest number.
#
#Example 1:
#
#Input: [10,2]
#Output: "210"
#Example 2:
#
#Input: [3,30,34,5,9]
#Output: "9534330"
#Note: The result may be very large, so you need to return a string instead of an integer.

class CustomCompKey(str):
    def __lt__(x, y):
        return x + y > y + x

class Solution:
    def largestNumber(self, nums: List[int]) -> str:
        nums = map(str, nums)
        cmp = lambda x, y : -1 if x + y > y + x else 1 if x + y < y + x else 0
        nums = sorted(nums, key = cmp_to_key(cmp))
        #nums = sorted(nums, key = CustomCompKey)
        ans = ''.join(nums)
        return '0' if ans[0] == '0' else ans
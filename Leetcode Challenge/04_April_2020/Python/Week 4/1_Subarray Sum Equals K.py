#Given an array of integers and an integer k, you need to find the total number of continuous subarrays whose sum equals to k.
#
#Example 1:
#Input:nums = [1,1,1], k = 2
#Output: 2
#Note:
#The length of the array is in range [1, 20,000].
#The range of numbers in the array is [-1000, 1000] and the range of the integer k is [-1e7, 1e7].
#   Hide Hint #1  
#Will Brute force work here? Try to optimize it.
#   Hide Hint #2  
#Can we optimize it by using some extra space?
#   Hide Hint #3  
#What about storing sum frequencies in a hash table? Will it be useful?
#   Hide Hint #4  
#sum(i,j)=sum(0,j)-sum(0,i), where sum(i,j) represents the sum of all the elements from index i to j-1. Can we use this property to optimize it.

class Solution:
    def subarraySum(self, nums: List[int], k: int) -> int:
        if not nums:
            return 0
        sums = defaultdict(lambda : 0)
        #sums = {}
        current_sum = 0
        ans = 0
        for e in nums:
            current_sum += e
            if current_sum == k:
                ans += 1
            if (current_sum - k) in sums:
                ans += sums[current_sum - k]
            #if current_sum in sums:
            sums[current_sum] += 1
            #else:
                #sums[current_sum] = 1
        return ans
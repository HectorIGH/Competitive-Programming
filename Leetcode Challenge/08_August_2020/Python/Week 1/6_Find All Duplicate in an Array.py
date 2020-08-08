#Given an array of integers, 1 ≤ a[i] ≤ n (n = size of array), some elements appear twice and others appear once.
#
#Find all the elements that appear twice in this array.
#
#Could you do it without extra space and in O(n) runtime?
#
#Example:
#Input:
#[4,3,2,7,8,2,3,1]
#
#Output:
#[2,3]

class Solution:
    def findDuplicates(self, nums: List[int]) -> List[int]:
        '''
        #a = 0b1111111110
        #a = (1 << (len(nums) + 1)) - 2
        a = (2 << len(nums)) - 2
        b, zet, i = 0, 0, 0
        ans = []
        for e in nums:
            c = 1 << e
            a ^= c #(1 << e)
            b |= c #(1 << e)
        zet = a & b
        while zet:
            if zet & 1:
                ans.append(i)
            i += 1
            zet >>= 1
        return ans
        '''
        zet = 0 #set()
        ans = []
        for i in range(len(nums)):
            num = nums[i]
            #if not num in zet:
            if (1 << num) & zet == 0:
                #zet.add(num)
                zet ^= (1 << num)
            else:
                ans.append(num)
        return ans
        '''
        ans = []
        for n in nums:
            if nums[abs(n) - 1] < 0:
                ans.append(abs(n))
            else:
                nums[abs(n) - 1] *= -1
        return ans
        '''
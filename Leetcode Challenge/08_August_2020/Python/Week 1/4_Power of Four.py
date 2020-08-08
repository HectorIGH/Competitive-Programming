#Given an integer (signed 32 bits), write a function to check whether it is a power of 4.
#
#Example 1:
#
#Input: 16
#Output: true
#Example 2:
#
#Input: 5
#Output: false
#Follow up: Could you solve it without loops/recursion?

class Solution:
    def isPowerOfFour(self, num: int) -> bool:
        '''
        i = 0
        while num:
            if num & 1:
                break
            i += 1
            num >>= 1
        return num == 1 and (i & 1) == 0
        '''
        '''
        return num > 0 and (num == 4 ** (int)(math.log(num) / math.log(4)))
        '''
        return num > 0 and num & (num - 1) == 0 and num & 0x55555555
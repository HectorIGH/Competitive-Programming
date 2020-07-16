#Given a positive integer num, write a function which returns True if num is a perfect square else False.
#
#Note: Do not use any built-in library function such as sqrt.
#
#Example 1:
#
#Input: 16
#Output: true
#Example 2:
#
#Input: 14
#Output: false

class Solution:
    def isPerfectSquare(self, num: int) -> bool:
        if num < 2:
            return True
        
        # Exhaustive search 
        #i = 2
        #while i * i <= num:
        #    if i * i == num:
        #        return True
        #    i += 1
        #return False
    
        # Binary search
        l, r = 2, num // 2
        while l <= r:
            mid = l + (r - l) // 2
            sq = mid * mid
            if sq == num:
                return True
            if sq > num:
                r = mid - 1
            else:
                l = mid + 1
        return False
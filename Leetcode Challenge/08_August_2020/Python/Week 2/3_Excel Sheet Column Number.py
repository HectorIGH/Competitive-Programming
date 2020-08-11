#Given a column title as appear in an Excel sheet, return its corresponding column number.
#
#For example:
#
#    A -> 1
#    B -> 2
#    C -> 3
#    ...
#    Z -> 26
#    AA -> 27
#    AB -> 28 
#    ...
#Example 1:
#
#Input: "A"
#Output: 1
#Example 2:
#
#Input: "AB"
#Output: 28
#Example 3:
#
#Input: "ZY"
#Output: 701
# 
#
#Constraints:
#
#1 <= s.length <= 7
#s consists only of uppercase English letters.
#s is between "A" and "FXSHRXW".

class Solution:
    def titleToNumber(self, s: str) -> int:
        # A == 65
        #p = len(s) - 1
        p = 1
        ans = 0
        '''
        for n in s:
            ans += (ord(n) - 64) * 26 ** p
            p -= 1
        '''
        for n in reversed(s):
            ans += (ord(n) - 64) * p
            p *= 26
        
        return ans
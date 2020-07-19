#Given two binary strings, return their sum (also a binary string).
#
#The input strings are both non-empty and contains only characters 1 or 0.
#
#Example 1:
#
#Input: a = "11", b = "1"
#Output: "100"
#Example 2:
#
#Input: a = "1010", b = "1011"
#Output: "10101"
# 
#
#Constraints:
#
#Each string consists only of '0' or '1' characters.
#1 <= a.length, b.length <= 10^4
#Each string is either "0" or doesn't contain any leading zero.

class Solution:
    def addBinary(self, a: str, b: str) -> str:
        #return bin(int(a, 2) + int(b, 2))[2:]
        '''
        a | b | c | r | -> c
        0 | 0 | 0 | 0 | 0
        0 | 0 | 1 | 1 | 0
        0 | 1 | 0 | 1 | 0
        0 | 1 | 1 | 0 | 1
        1 | 0 | 0 | 1 | 0
        1 | 0 | 1 | 0 | 1
        1 | 1 | 0 | 0 | 1
        1 | 1 | 1 | 1 | 1
        '''
        carry = 0
        i = len(a) - 1
        j = len(b) - 1
        ans = ""
        while i >= 0 or j >= 0 or carry:
            suma = carry
            suma += int(a[i]) if i >= 0 else 0
            suma += int(b[j]) if j >= 0 else 0
            ans += str(suma & 1) # % 2
            carry = suma >> 1 # //2
            i -= 1
            j -= 1
        return ans[::-1]
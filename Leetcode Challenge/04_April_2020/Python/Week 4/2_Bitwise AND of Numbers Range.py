#Given a range [m, n] where 0 <= m <= n <= 2147483647, return the bitwise AND of all numbers in this range, inclusive.
#
#Example 1:
#
#Input: [5,7]
#Output: 4
#Example 2:
#
#Input: [0,1]
#Output: 0

class Solution:
    def rangeBitwiseAnd(self, m: int, n: int) -> int:
        #if n&m:
            #maxi = n
            #bm = 1 << (int(log2(m)) + 1)
            #bn = 1 << (int(log2(n)) + 1)
            #if m <= bm <= n:
                #maxi = bm
            #elif m <= bn <= n:
                #maxi = bn
            #ans = maxi
            #for i in range(maxi, m - 1, -1):
                #ans &= i
                #if ans == 0:
                    #break
        #else:
            #return 0
        #return ans
        if n&m:
            count = 0
            while m != n and n and m:
                m >>= 1
                n >>= 1
                count += 1
            return m << count
        else:
            return 0
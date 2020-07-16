#Given a positive integer n, find the least number of perfect square numbers (for example, 1, 4, 9, 16, ...) which sum to n.
#
#Example 1:
#
#Input: n = 12
#Output: 3 
#Explanation: 12 = 4 + 4 + 4.
#Example 2:
#
#Input: n = 13
#Output: 2
#Explanation: 13 = 4 + 9.

class Solution:
    def numSquares(self, n: int) -> int:
        '''
        # DP
        if n < 4:
            return n
        dp = [0] * (n + 1)
        dp[1] = 1
        dp[2] = 2
        dp[3] = 3
        for i in range(4, n + 1):
            dp[i] = i
            j = 0
            while j * j <= i:
                dp[i] = min(dp[i], dp[i - j * j] + 1)
                j += 1
        return dp[n]
        '''
        #Lagrange
        if n == int(sqrt(n)) ** 2:
            return 1
        for i in range(int(sqrt(n)) + 1):
            c = n - i * i
            if c == int(sqrt(c)) ** 2:
                return 2
        while n % 4 == 0:
            n >>= 2
        if n % 8 == 7:
            return 4
        
        return 3
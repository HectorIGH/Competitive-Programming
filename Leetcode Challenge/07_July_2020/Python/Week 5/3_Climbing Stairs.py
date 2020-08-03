#You are climbing a stair case. It takes n steps to reach to the top.
#
#Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
#
#Example 1:
#
#Input: 2
#Output: 2
#Explanation: There are two ways to climb to the top.
#1. 1 step + 1 step
#2. 2 steps
#Example 2:
#
#Input: 3
#Output: 3
#Explanation: There are three ways to climb to the top.
#1. 1 step + 1 step + 1 step
#2. 1 step + 2 steps
#3. 2 steps + 1 step
# 
#
#Constraints:
#
#1 <= n <= 45
#   Hide Hint #1  
#To reach nth step, what could have been your previous steps? (Think about the step sizes)

class Solution:
    def climbStairs(self, n: int) -> int:
        if n <= 2:
            return n
        b = [0] * (n + 1) # avoid the shifted indexes
        b[1] = 1
        b[2] = 2
        for i in range(3, n + 1):
            b[i] = b[i-1] + b[i-2]
        return b[n]
    '''
        self.memo = {1: 1, 2: 2}
        return self.aux(n)
    
    def aux(self, n):
        if n in self.memo:
            return self.memo[n]
        else:
            self.memo[n - 1] = self.aux(n - 1)
            self.memo[n - 2] = self.aux(n - 2)
            return self.memo[n - 1] + self.memo[n - 2]
    '''
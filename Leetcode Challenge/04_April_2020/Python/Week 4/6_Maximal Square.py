#Given a 2D binary matrix filled with 0's and 1's, find the largest square containing only 1's and return its area.
#
#Example:
#
#Input: 
#
#1 0 1 0 0
#1 0 1 1 1
#1 1 1 1 1
#1 0 0 1 0
#
#Output: 4

class Solution:
    def maximalSquare(self, matrix: List[List[str]]) -> int:
        if not matrix:
            return 0
        m = len(matrix)
        n = len(matrix[0])
        dp = [[0 for _ in range(n)] for _ in range(m)]
        side = 0
        for i in range(m):
            for j in range(n):
                if matrix[i][j] == "1":
                    dp[i][j] = 1 + min(dp[i][j-1], dp[i-1][j], dp[i-1][j-1])
                    side = max(side, dp[i][j])
                else:
                    dp[i][j] = 0
        return pow(side, 2)
#Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right which minimizes the sum of all numbers along its path.
#
#Note: You can only move either down or right at any point in time.
#
#Example:
#
#Input:
#[
#  [1,3,1],
#  [1,5,1],
#  [4,2,1]
#]
#Output: 7
#Explanation: Because the path 1→3→1→1→1 minimizes the sum.

class Solution:
    def minPathSum(self, grid: List[List[int]]) -> int:
        if not grid:
            return 0
        
        def finder(grid, cost, row, col):
            rows, cols = len(grid), len(grid[0])
            if row == rows or col == cols:
                return float('inf')
            elif cost[row][col] != -1:
                return cost[row][col]
            else:
                right, down = finder(grid, cost, row, col + 1), finder(grid, cost, row + 1, col)
                cost[row][col] = min(right, down) + grid[row][col]
            return cost[row][col]
        
        rows, cols = len(grid), len(grid[0])
        cost = [[-1] * cols for _ in range(rows)]
        cost[rows-1][cols-1] = grid[rows-1][cols-1]
        path = finder(grid, cost, 0, 0)
        return path
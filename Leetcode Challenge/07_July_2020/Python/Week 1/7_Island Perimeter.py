#You are given a map in form of a two-dimensional integer grid where 1 represents land and 0 represents water.
#
#Grid cells are connected horizontally/vertically (not diagonally). The grid is completely surrounded by water, and there is exactly one island (i.e., one or more connected land cells).
#
#The island doesn't have "lakes" (water inside that isn't connected to the water around the island). One cell is a square with side length 1. The grid is rectangular, width and height don't exceed 100. Determine the perimeter of the island.
#
# 
#
#Example:
#
#Input:
#[[0,1,0,0],
# [1,1,1,0],
# [0,1,0,0],
# [1,1,0,0]]
#
#Output: 16
#
#Explanation: The perimeter is the 16 yellow stripes in the image below:
#

class Solution:
    def islandPerimeter(self, grid: List[List[int]]) -> int:
        # Iterative
        rows, cols, p = len(grid), len(grid[0]), 0
        for i in range(rows):
            for j in range(cols):
                if grid[i][j]:
                    p += 4
                    if i < rows - 1:
                        p -= 2 * grid[i + 1][j]
                    if j < cols - 1:
                        p -= 2 * grid[i][j + 1]
        return p
        '''
        # DFS
        r, c = len(grid), len(grid[0])
        for i in range(r):
            for j in range(c):
                if grid[i][j]:
                    return self.dfs(grid, i, j)
        
    def dfs(self, grid, i, j):
        if i < 0 or i >= len(grid) or j < 0 or j >= len(grid[0]):
            return 1
        if grid[i][j] == 0:
            return 1
        if grid[i][j] == -1:
            return 0
        grid[i][j] = -1
        p = 0
        p += self.dfs(grid, i, j + 1)
        p += self.dfs(grid, i + 1, j)
        p += self.dfs(grid, i, j - 1)
        p += self.dfs(grid, i - 1, j)
        return p
    '''
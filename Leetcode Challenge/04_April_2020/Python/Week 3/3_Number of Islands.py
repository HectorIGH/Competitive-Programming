#Given a 2d grid map of '1's (land) and '0's (water), count the number of islands. An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.
#
#Example 1:
#
#Input:
#11110
#11010
#11000
#00000
#
#Output: 1
#Example 2:
#
#Input:
#11000
#11000
#00100
#00011
#
#Output: 3

class Solution:
    def numIslands(self, grid: List[List[str]]) -> int:
        if not grid:
            return 0
        grid = [[int(i) for i in row] for row in grid]
        rows, cols = len(grid), len(grid[0])
        counter = 0
        maxSize = 0

        def DFS(grid, row, col):
            if row < 0 or row >= len(grid) or col < 0 or col >= len(grid[0]):
                return 0
            if grid[row][col] in [0, 2]: #Water or visited land
                return 0

            grid[row][col] = 2 #Visited
            size = 1
            #For loops for 8 conneted
            #for r in [row - 1, row + 1]:
            #	for c in [col - 1, col + 1]:
            #		size += DFS(grid, r, c)
            #For 4 connected
            for r, c in [(row, col - 1), (row, col + 1), (row - 1, col), (row + 1, col)]:
                size += DFS(grid, r, c)
            #size +=DFS(grid, row, col - 1)
            #size +=DFS(grid, row, col + 1)
            #size +=DFS(grid, row - 1, col)
            #size +=DFS(grid, row + 1, col)
            return size

        for row in range(rows):
            for col in range(cols):
                if grid[row][col] == 1: #Land
                    size = DFS(grid, row, col) #Size of the island
                    if size:
                        counter +=  1
                    #maxSize =max(maxSize, size)
        return counter
#In a given grid, each cell can have one of three values:
#
#the value 0 representing an empty cell;
#the value 1 representing a fresh orange;
#the value 2 representing a rotten orange.
#Every minute, any fresh orange that is adjacent (4-directionally) to a rotten orange becomes rotten.
#
#Return the minimum number of minutes that must elapse until no cell has a fresh orange.  If this is impossible, return -1 instead.
#
# 
#
#Example 1:
#
#
#
#Input: [[2,1,1],[1,1,0],[0,1,1]]
#Output: 4
#Example 2:
#
#Input: [[2,1,1],[0,1,1],[1,0,1]]
#Output: -1
#Explanation:  The orange in the bottom left corner (row 2, column 0) is never rotten, because rotting only happens 4-directionally.
#Example 3:
#
#Input: [[0,2]]
#Output: 0
#Explanation:  Since there are already no fresh oranges at minute 0, the answer is just 0.
# 
#
#Note:
#
#1 <= grid.length <= 10
#1 <= grid[0].length <= 10
#grid[i][j] is only 0, 1, or 2.

class Solution:
    def orangesRotting(self, grid: List[List[int]]) -> int:
        rows = len(grid)
        cols = len(grid[0])
        q = deque()
        fresh = 0
        dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]]
        days = 0
        for i in range(rows):
            for j in range(cols):
                if grid[i][j] == 2:
                    q.append((i, j))
                if grid[i][j] == 1:
                    fresh += 1
        
        while q and fresh > 0:
            days += 1
            for _ in range(len(q)):
                x, y = q.popleft()
                for dx, dy in dirs:
                    if 0 <= x + dx < rows and 0 <= y + dy < cols and grid[x + dx][y + dy] == 1:
                        fresh -= 1
                        grid[x + dx][y + dy] = 2
                        q.append((x + dx, y + dy))
        return -1 if fresh else days
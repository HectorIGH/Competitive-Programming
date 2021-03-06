//On a 2-dimensional grid, there are 4 types of squares:
//
//1 represents the starting square.  There is exactly one starting square.
//2 represents the ending square.  There is exactly one ending square.
//0 represents empty squares we can walk over.
//-1 represents obstacles that we cannot walk over.
//Return the number of 4-directional walks from the starting square to the ending square, that walk over every non-obstacle square exactly once.
//
// 
//
//Example 1:
//
//Input: [[1,0,0,0],[0,0,0,0],[0,0,2,-1]]
//Output: 2
//Explanation: We have the following two paths: 
//1. (0,0),(0,1),(0,2),(0,3),(1,3),(1,2),(1,1),(1,0),(2,0),(2,1),(2,2)
//2. (0,0),(1,0),(2,0),(2,1),(1,1),(0,1),(0,2),(0,3),(1,3),(1,2),(2,2)
//Example 2:
//
//Input: [[1,0,0,0],[0,0,0,0],[0,0,0,2]]
//Output: 4
//Explanation: We have the following four paths: 
//1. (0,0),(0,1),(0,2),(0,3),(1,3),(1,2),(1,1),(1,0),(2,0),(2,1),(2,2),(2,3)
//2. (0,0),(0,1),(1,1),(1,0),(2,0),(2,1),(2,2),(1,2),(0,2),(0,3),(1,3),(2,3)
//3. (0,0),(1,0),(2,0),(2,1),(2,2),(1,2),(1,1),(0,1),(0,2),(0,3),(1,3),(2,3)
//4. (0,0),(1,0),(2,0),(2,1),(1,1),(0,1),(0,2),(0,3),(1,3),(1,2),(2,2),(2,3)
//Example 3:
//
//Input: [[0,1],[2,0]]
//Output: 0
//Explanation: 
//There is no path that walks over every empty square exactly once.
//Note that the starting and ending square can be anywhere in the grid.
// 
//
//Note:
//
//1 <= grid.length * grid[0].length <= 20

class Solution {
    int ans;
    public int uniquePathsIII(int[][] grid) {
        ans = 0;
        int empty = 0;
        int m = grid[0].length;
        int n = grid.length;
        int sx = 0;
        int sy = 0;
        for(int i = 0; i < n; i++) {
            for(int j = 0; j < m; j++) {
                if(grid[i][j] == 1) {
                    sx = i;
                    sy = j;
                }
                empty += grid[i][j] != -1 ? 1 : 0;
            }
        }
        dfs(sx, sy, empty - 1, n, m, grid);
        return ans;
    }
    public void dfs(int x, int y, int rest, int n, int m, int[][] grid) {
        if(x < 0 || x >= n || y < 0 || y >= m || grid[x][y] < 0) {
            return ;
        }
        if(grid[x][y] == 2 && rest == 0) {
            ans += 1;
        }
        int[][] neibs = {{0, 1}, {0, -1}, {1, 0}, {-1, 0}}; //[[0, 1], [0, -1], [1, 0], [-1, 0]];
        int dx, dy;
        int save;
        for(int[] neib : neibs) {
            dx = neib[0];
            dy = neib[1];
            save = grid[x][y];
            grid[x][y] = -2;
            dfs(x + dx, y + dy, rest - 1, n, m, grid);
            grid[x][y] = save;
        }
    }
}
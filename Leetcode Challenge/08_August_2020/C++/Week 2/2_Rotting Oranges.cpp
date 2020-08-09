//In a given grid, each cell can have one of three values:
//
//the value 0 representing an empty cell;
//the value 1 representing a fresh orange;
//the value 2 representing a rotten orange.
//Every minute, any fresh orange that is adjacent (4-directionally) to a rotten orange becomes rotten.
//
//Return the minimum number of minutes that must elapse until no cell has a fresh orange.  If this is impossible, return -1 instead.
//
// 
//
//Example 1:
//
//
//
//Input: [[2,1,1],[1,1,0],[0,1,1]]
//Output: 4
//Example 2:
//
//Input: [[2,1,1],[0,1,1],[1,0,1]]
//Output: -1
//Explanation:  The orange in the bottom left corner (row 2, column 0) is never rotten, because rotting only happens 4-directionally.
//Example 3:
//
//Input: [[0,2]]
//Output: 0
//Explanation:  Since there are already no fresh oranges at minute 0, the answer is just 0.
// 
//
//Note:
//
//1 <= grid.length <= 10
//1 <= grid[0].length <= 10
//grid[i][j] is only 0, 1, or 2.

class Solution {
public:
    int orangesRotting(vector<vector<int>>& grid) {
        int rows = grid.size();
        int cols = grid[0].size();
        queue<vector<int>> q;
        int fresh = 0;
        vector<vector<int>> dirs = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        int days = 0;
        for(int i = 0; i < rows; i++) {
            for(int j = 0; j < cols; j++) {
                if(grid[i][j] == 2) {
                    q.push({i, j});
                }
                if(grid[i][j] == 1) {
                    fresh++;
                }
            }
        }
        
        while(!q.empty() && fresh > 0) {
            days++;
            int size = q.size();
            while(size > 0) {
                vector<int> xy = q.front();
                q.pop();
                for(vector<int> dxy : dirs) {
                    int x = xy[0] + dxy[0];
                    int y = xy[1] + dxy[1];
                    if(0 <= x && x < rows && 0 <= y && y < cols && grid[x][y] == 1) {
                        fresh--;
                        grid[x][y] = 2;
                        q.push({x, y});
                    }
                }
                size--;
            }
        }
        return fresh != 0 ? -1 : days;
    }
};
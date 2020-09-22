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

impl Solution {
    pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut ans: i32 = 0;
        let mut empty: i32 = 0;
        let mut m: usize = grid[0].len();
        let mut n: usize = grid.len();
        let mut sx: usize = 0;
        let mut sy: usize = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    sx = i;
                    sy = j;
                }
                empty += if grid[i][j] != -1 {1} else {0};
            }
        }
        Solution::dfs(sx, sy, empty - 1, n, m, &mut grid, &mut ans);
        return ans;
    }
    
    pub fn dfs(x: usize, y: usize, rest: i32, n: usize, m: usize, mut grid: &mut Vec<Vec<i32>>, ans: &mut i32) {
        if x < 0 || x >= n || y < 0 || y >= m || grid[x][y] < 0 {
            return ;
        }
        if grid[x][y] == 2 && rest == 0 {
            *ans += 1;
        }
        let neibs: Vec<Vec<i32>> = vec![vec![0, 1], vec![0, -1], vec![1, 0], vec![-1, 0]];
        let mut dx: usize = 0;
        let mut dy: usize = 0;
        let mut save: i32 = 0;
        for neib in neibs {
            dx = neib[0] as usize;
            dy = neib[1] as usize;
            save = grid[x][y];
            grid[x][y] = -2;
            Solution::dfs(x + dx, y + dy, rest - 1, n, m, &mut grid, ans);
            grid[x][y] = save;
        }
    }
}
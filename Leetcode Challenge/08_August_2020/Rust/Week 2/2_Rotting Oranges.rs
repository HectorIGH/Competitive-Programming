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

use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut rows = grid.len();
        let mut cols = grid[0].len();
        let mut q: VecDeque<Vec<i32>> = VecDeque::new();
        let mut fresh = 0;
        let mut dirs = vec![vec![1, 0], vec![-1, 0], vec![0, 1], vec![0, -1]];
        let mut days = 0;
        
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == 2 {
                    q.push_back(vec![i as i32, j as i32]);
                }
                if grid[i][j] == 1 {
                    fresh += 1;
                }
            }
        }
        let mut rows:i32 = rows as i32;
        let mut cols:i32 = cols as i32;
        while(!q.is_empty() && fresh > 0) {
            days += 1;
            let mut size = q.len();
            while size > 0 {
                let mut xy: Vec<i32> = q.pop_front().unwrap();
                for dxy in &dirs {
                    let mut x = xy[0] + dxy[0];
                    let mut y = xy[1] + dxy[1];
                    if 0 <= x && x < rows && 0 <= y && y < cols && grid[x as usize][y as usize] == 1 {
                        fresh -= 1;
                        grid[x as usize][y as usize] = 2;
                        q.push_back(vec![x, y]);
                    }
                }
                size -= 1;
            }
        }
        return if fresh != 0 {-1} else {days};
    }
}
//Given a 2D board containing 'X' and 'O' (the letter O), capture all regions surrounded by 'X'.
//
//A region is captured by flipping all 'O's into 'X's in that surrounded region.
//
//Example:
//
//X X X X
//X O O X
//X X O X
//X O X X
//After running your function, the board should be:
//
//X X X X
//X X X X
//X X X X
//X O X X
//Explanation:
//
//Surrounded regions shouldn’t be on the border, which means that any 'O' on the border of the board are not flipped to 'X'. Any 'O' that is not on the border and it is not connected to an 'O' on the border will be flipped to 'X'. Two cells are connected if they are adjacent cells connected horizontally or vertically.

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.len() == 0 {
            return;
        }
        let mut n = board.len();
        let mut m = board[0].len();
        for i in 0..n {
            if board[i][0] == 'O' {
                dfs(board, i, 0);
            }
            if board[i][m-1] == 'O' {
                dfs(board, i, m-1);
            }
        }
        for i in 0..m {
            if board[0][i] == 'O' {
                dfs(board, 0, i);
            }
            if board[n-1][i] == 'O' {
                dfs(board, n-1, i);
            }
        }
        for i in 0..n {
            for j in 0..m {
                if board[i][j] == 'O' {
                    board[i][j] = 'X';
                }
                if board[i][j] == '#' {
                    board[i][j] = 'O';
                }
            }
        }
    }
}
pub fn dfs(board: &mut Vec<Vec<char>>, row: usize, col: usize) {
    if row >= 0 && row < board.len() && col >= 0 && col < board[0].len() && board[row][col] == 'O' {
        board[row][col] = '#';
        dfs(board, row + 1, col);
        dfs(board, row, col + 1);
        dfs(board, row - 1, col);
        dfs(board, row, col - 1);
    } else {
        return;
    }
}
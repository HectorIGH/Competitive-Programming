//Given a 2D board and a word, find if the word exists in the grid.
//
//The word can be constructed from letters of sequentially adjacent cell, where "adjacent" cells are those horizontally or vertically neighboring. The same letter cell may not be used more than once.
//
//Example:
//
//board =
//[
//  ['A','B','C','E'],
//  ['S','F','C','S'],
//  ['A','D','E','E']
//]
//
//Given word = "ABCCED", return true.
//Given word = "SEE", return true.
//Given word = "ABCB", return false.
// 
//
//Constraints:
//
//board and word consists only of lowercase and uppercase English letters.
//1 <= board.length <= 200
//1 <= board[i].length <= 200
//1 <= word.length <= 10^3

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut w : Vec<char> = word.chars().collect();
        let mut board = board;
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if w[0] == board[i][j] && Solution::dfs(i, j, 0, &mut board, &mut w) {
                    return true;
                }
            }
        }
        return false;
    }
    
    pub fn dfs(i : usize, j : usize, id : usize, board : &mut Vec<Vec<char>>, word : &mut Vec<char>) -> bool {
        if id == (word.len() - 1) {
            return true;
        }
        let a = board[i][j];
        board[i][j] = '#';
        if i > 0 && board[i - 1][j] == word[id + 1] && Solution::dfs(i - 1, j, id + 1, board, word) {
            return true;
        }
        if j > 0 && board[i][j - 1] == word[id + 1] && Solution::dfs(i, j - 1, id + 1, board, word) {
            return true;
        }
        if (i < (board.len() - 1)) && board[i + 1][j] == word[id + 1] && Solution::dfs(i + 1, j, id + 1, board, word) {
            return true;
        }
        if (j < (board[0].len() - 1)) && board[i][j + 1] == word[id + 1] && Solution::dfs(i, j + 1, id + 1, board, word) {
            return true;
        }
        board[i][j] = a;
        return false;
    }
}
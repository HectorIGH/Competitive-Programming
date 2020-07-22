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

class Solution {
    public boolean exist(char[][] board, String word) {
        for(int i = 0; i < board.length; i++) {
            for(int j = 0; j < board[0].length; j++) {
                if(word.charAt(0) == board[i][j] && dfs(i, j, 0, board, word)) {
                    return true;
                }
            }
        }
        return false;
    }
    
    public boolean dfs(int i, int j, int id, char[][] board, String word) {
        if(id == word.length() - 1) {
            return true;
        }
        board[i][j] -= 65;
        if(i > 0 && board[i - 1][j] == word.charAt(id + 1) && dfs(i - 1, j, id + 1, board, word)) {
            return true;
        }
        if(j > 0 && board[i][j - 1] == word.charAt(id + 1) && dfs(i, j - 1, id + 1, board, word)) {
            return true;
        }
        if(i < board.length - 1 && board[i + 1][j] == word.charAt(id + 1) && dfs(i + 1, j, id + 1, board, word)) {
            return true;
        }
        if(j < board[0].length - 1 && board[i][j + 1] == word.charAt(id + 1) && dfs(i, j + 1, id + 1, board, word)) {
            return true;
        }
        board[i][j] += 65;
        return false;
    }
}
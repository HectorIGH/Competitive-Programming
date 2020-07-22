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
public:
    bool exist(vector<vector<char>>& board, string word) {
        for(int i = 0; i < board.size(); i++) {
            for(int j = 0; j < board[0].size(); j++) {
                if(word[0] == board[i][j] && dfs(i, j, 0, board, word)) {
                    return true;
                }
            }
        }
        return false;
    }
    
    bool dfs(int i, int j, int id, vector<vector<char>> board, string word) {
        if(id == word.size() - 1) {
            return true;
        }
        board[i][j] -= 65;
        if(i > 0 && board[i - 1][j] == word[id + 1] && dfs(i - 1, j, id + 1, board, word)) {
            return true;
        }
        if(j > 0 && board[i][j - 1] == word[id + 1] && dfs(i, j - 1, id + 1, board, word)) {
            return true;
        }
        if(i < board.size() - 1 && board[i + 1][j] == word[id + 1] && dfs(i + 1, j, id + 1, board, word)) {
            return true;
        }
        if(j < board[0].size() - 1 && board[i][j + 1] == word[id + 1] && dfs(i, j + 1, id + 1, board, word)) {
            return true;
        }
        board[i][j] += 65;
        return false;
    }
};
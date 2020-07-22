#Given a 2D board and a word, find if the word exists in the grid.
#
#The word can be constructed from letters of sequentially adjacent cell, where "adjacent" cells are those horizontally or vertically neighboring. The same letter cell may not be used more than once.
#
#Example:
#
#board =
#[
#  ['A','B','C','E'],
#  ['S','F','C','S'],
#  ['A','D','E','E']
#]
#
#Given word = "ABCCED", return true.
#Given word = "SEE", return true.
#Given word = "ABCB", return false.
# 
#
#Constraints:
#
#board and word consists only of lowercase and uppercase English letters.
#1 <= board.length <= 200
#1 <= board[i].length <= 200
#1 <= word.length <= 10^3

class Solution:
    def exist(self, board: List[List[str]], word: str) -> bool:
        self.board = board
        self.word = word
        self.size = len(word) - 1
        self.rows = len(board)
        self.cols = len(board[0])
        for i in range(self.rows):
            for j in range(self.cols):
                if word[0] == board[i][j] and self.dfs(i, j, 0):
                    return True
        return False
    
    def dfs(self, i, j, idx):
        if idx == self.size:
            return True
        a = self.board[i][j]
        self.board[i][j] = '#' #chr(ord(self.board[i][j]) - 65)
        if i > 0 and self.board[i - 1][j] == self.word[idx + 1] and self.dfs(i - 1, j, idx + 1):
            return True
        if j > 0 and self.board[i][j - 1] == self.word[idx + 1] and self.dfs(i, j - 1, idx + 1):
            return True
        if i < self.rows - 1 and self.board[i + 1][j] == self.word[idx + 1] and self.dfs(i + 1, j, idx + 1):
            return True
        if j < self.cols - 1 and self.board[i][j + 1] == self.word[idx + 1] and self.dfs(i, j + 1, idx + 1):
            return True
        self.board[i][j] = a # chr(ord(self.board[i][j]) + 65)
        return False
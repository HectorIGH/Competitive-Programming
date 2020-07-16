#Given a 2D board and a list of words from the dictionary, find all words in the board.
#
#Each word must be constructed from letters of sequentially adjacent cell, where "adjacent" cells are those horizontally or vertically neighboring. The same letter cell may not be used more than once in a word.
#
# 
#
#Example:
#
#Input: 
#board = [
#  ['o','a','a','n'],
#  ['e','t','a','e'],
#  ['i','h','k','r'],
#  ['i','f','l','v']
#]
#words = ["oath","pea","eat","rain"]
#
#Output: ["eat","oath"]
# 
#
#Note:
#
#All inputs are consist of lowercase letters a-z.
#The values of words are distinct.
#   Hide Hint #1  
#You would need to optimize your backtracking to pass the larger test. Could you stop backtracking earlier?
#   Hide Hint #2  
#If the current candidate does not exist in all words' prefix, you could stop backtracking immediately. What kind of data structure could answer such query efficiently? Does a hash table work? Why or why not? How about a Trie? If you would like to learn how to implement a basic trie, please work on this problem: Implement Trie (Prefix Tree) first.

class Trie:
    def __init__(self):
        self.children = {} #[None] * 26
        self.end_of_word = False
        
    def insert(self, word:str):
        current = self
        for c in word:
            idx = ord(c) - ord('a')
            if idx not in current.children:
                current.children[idx] = Trie()
            current = current.children[idx]
        current.end_of_word = True
    

class Solution:
    def findWords(self, board: List[List[str]], words: List[str]) -> List[str]:
        if not words:
            return []
        self.board = board
        self.result = set()
        trie = Trie()
        for word in words:
            trie.insert(word)
        for i in range(len(board)):
            for j in range(len(board[0])):
                self.dfs(i, j, trie, "")
        return list(self.result)
    def dfs(self, i, j, trie, s):
        c = self.board[i][j]
        if c == '#':
            return
        self.board[i][j] = '#'
        #t = trie.children[ord(c) - ord('a')]
        idx = ord(c) - ord('a')
        if idx in trie.children:
            t = trie.children[idx]
            ss = s + c
            if t.end_of_word:
                self.result.add(ss)
            if i < len(self.board) - 1:
                self.dfs(i + 1, j, t, ss)
            if i > 0:
                self.dfs(i - 1, j, t, ss)
            if j < len(self.board[0]) - 1:
                self.dfs(i, j + 1, t, ss)
            if j > 0:
                self.dfs(i, j - 1, t, ss)
        self.board[i][j] = c
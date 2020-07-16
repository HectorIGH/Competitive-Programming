//Given a 2D board and a list of words from the dictionary, find all words in the board.
//
//Each word must be constructed from letters of sequentially adjacent cell, where "adjacent" cells are those horizontally or vertically neighboring. The same letter cell may not be used more than once in a word.
//
// 
//
//Example:
//
//Input: 
//board = [
//  ['o','a','a','n'],
//  ['e','t','a','e'],
//  ['i','h','k','r'],
//  ['i','f','l','v']
//]
//words = ["oath","pea","eat","rain"]
//
//Output: ["eat","oath"]
// 
//
//Note:
//
//All inputs are consist of lowercase letters a-z.
//The values of words are distinct.
//   Hide Hint #1  
//You would need to optimize your backtracking to pass the larger test. Could you stop backtracking earlier?
//   Hide Hint #2  
//If the current candidate does not exist in all words' prefix, you could stop backtracking immediately. What kind of data structure could answer such query efficiently? Does a hash table work? Why or why not? How about a Trie? If you would like to learn how to implement a basic trie, please work on this problem: Implement Trie (Prefix Tree) first.

class Trie {
public:
    Trie* children[26];
    bool endOfWord;
    Trie():endOfWord(false) {
        for(int i = 0; i < 26; i++) {
            children[i] = nullptr;
        }
    }
    ~Trie() {
        for(int i = 0; i < 26; i++) {
            if(children[i]) {
                delete children[i];
            }
        }
    }
    void insert(string word) {
        Trie* current = this;
        for(char c : word) {
            if(!current->children[c - 'a']) {
                current->children[c - 'a'] = new Trie();
            }
            current = current->children[c - 'a'];
        }
        current->endOfWord = true;
    }
};

class Solution {
public:
    vector<string> findWords(vector<vector<char>>& board, vector<string>& words) {
        if(words.size() == 0) {
            return {};
        }
        Trie trie;
        for(string word : words) {
            trie.insert(word);
        }
        unordered_set<string> res;
        for(int i = 0; i < board.size(); i++) {
            for(int j = 0; j < board[0].size(); j++) {
                dfs(board, i, j, &trie, res, "");
            }
        }
        vector<string> ans(res.begin(), res.end());
        return ans;
    }
    void dfs(vector<vector<char>>& board, int i, int j, Trie* trie, unordered_set<string>& res, string s) {
        char c = board[i][j];
        if(c == '#') {
            return;
        }
        board[i][j] = '#';
        Trie* t = trie->children[c - 'a'];
        if(t) {
            string ss = s + c;
            if(t->endOfWord) {
                res.insert(ss);
            }
            if(i < board.size() - 1) {
                dfs(board, i + 1, j, t, res, ss);
            }
            if(i > 0) {
                dfs(board, i - 1, j, t, res, ss);
            }
            if(j < board[0].size() - 1) {
                dfs(board, i, j + 1, t, res, ss);
            }
            if(j > 0) {
                dfs(board, i, j - 1, t, res, ss);
            }
        }
        board[i][j] = c;
    }
};
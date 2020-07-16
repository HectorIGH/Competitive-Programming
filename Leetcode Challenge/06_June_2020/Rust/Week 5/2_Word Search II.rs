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

use std::collections::HashSet;

#[derive(Default)]

struct Trie {
    children : [Option<Box<Trie>>; 26],
    word : Option<String>,
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie : Trie = Default::default();
        for word in words.iter() {
            let mut node = &mut trie;
            for c in word.as_bytes() {
                node = node.children[(c - b'a') as usize].get_or_insert(Box::new(Default::default()));
            }
            node.word = Some(word.clone());
        }
        let mut ans : HashSet<String> = HashSet::new();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                let mut visited : Vec<Vec<bool>> = vec![vec![false; board[0].len()]; board.len()];
                Solution::dfs(&board, i, j, &trie, &mut visited, &mut ans);
            }
        }
        ans.into_iter().collect()
    }
    fn dfs(
        board : &[Vec<char>],
        i : usize,
        j : usize,
        trie : &Trie,
        visited : &mut Vec<Vec<bool>>,
        ans : &mut HashSet<String>) {
            if visited[i][j] {
                return;
            }
            visited[i][j] = true;
            let c = board[i][j];
            if let Some(node) = &trie.children[(c as u8 - b'a') as usize] {
                if let Some(word) = &node.word {
                    ans.insert(word.clone());
                }
                if i < board.len() - 1 {
                    Solution::dfs(board, i + 1, j, node.as_ref(), visited, ans);
                }
                if i > 0 {
                    Solution::dfs(board, i - 1, j, node.as_ref(), visited, ans);
                }
                if j < board[0].len() - 1 {
                    Solution::dfs(board, i, j + 1, node.as_ref(), visited, ans);
                }
                if j > 0 {
                    Solution::dfs(board, i, j - 1, node.as_ref(), visited, ans);
                }
            }
            visited[i][j] = false;
    }
}
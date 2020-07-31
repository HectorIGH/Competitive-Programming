//Given a non-empty string s and a dictionary wordDict containing a list of non-empty words, add spaces in s to construct a sentence where each word is a valid dictionary word. Return all such possible sentences.
//
//Note:
//
//The same word in the dictionary may be reused multiple times in the segmentation.
//You may assume the dictionary does not contain duplicate words.
//Example 1:
//
//Input:
//s = "catsanddog"
//wordDict = ["cat", "cats", "and", "sand", "dog"]
//Output:
//[
//  "cats and dog",
//  "cat sand dog"
//]
//Example 2:
//
//Input:
//s = "pineapplepenapple"
//wordDict = ["apple", "pen", "applepen", "pine", "pineapple"]
//Output:
//[
//  "pine apple pen apple",
//  "pineapple pen apple",
//  "pine applepen apple"
//]
//Explanation: Note that you are allowed to reuse a dictionary word.
//Example 3:
//
//Input:
//s = "catsandog"
//wordDict = ["cats", "dog", "sand", "and", "cat"]
//Output:
//[]

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut memo : HashMap<usize, Vec<String>> = HashMap::new();
        let dict : HashSet<String> = HashSet::from_iter(word_dict.iter().cloned());
        Solution::dfs(&s, &dict, 0, &mut memo)
    }
    pub fn dfs(s : &String, dict : &HashSet<String>, idx : usize, memo : &mut HashMap<usize, Vec<String>>) -> Vec<String> {
        if memo.contains_key(&idx) {
            return memo[&idx].clone();
        }
        let mut res : Vec<String> = Vec::new();
        for i in idx..s.len() {
            let mut cur : String = s.clone().chars().skip(idx).take(i - idx + 1).collect();
            if dict.contains(&cur) {
                if i == (s.len() - 1) {
                    res.push(cur.clone());
                }
                let temp = Solution::dfs(s, dict, i + 1, memo);
                for e in &temp {
                    res.push(format!("{} {}", cur, e).to_string());
                }
            }
        }
        memo.insert(idx, res);
        memo[&idx].clone()
    }
}
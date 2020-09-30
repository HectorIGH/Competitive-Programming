//Given a non-empty string s and a dictionary wordDict containing a list of non-empty words, determine if s can be segmented into a space-separated sequence of one or more dictionary words.
//
//Note:
//
//The same word in the dictionary may be reused multiple times in the segmentation.
//You may assume the dictionary does not contain duplicate words.
//Example 1:
//
//Input: s = "leetcode", wordDict = ["leet", "code"]
//Output: true
//Explanation: Return true because "leetcode" can be segmented as "leet code".
//Example 2:
//
//Input: s = "applepenapple", wordDict = ["apple", "pen"]
//Output: true
//Explanation: Return true because "applepenapple" can be segmented as "apple pen apple".
//             Note that you are allowed to reuse a dictionary word.
//Example 3:
//
//Input: s = "catsandog", wordDict = ["cats", "dog", "sand", "and", "cat"]
//Output: false

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut cache: HashMap<String, bool> = HashMap::new();
        let wordSet: HashSet<String> = word_dict.iter().cloned().map(|a| a.to_string()).collect();
        let n:usize = s.len();
        return Solution::helper(0, n, &s, &mut cache, &wordSet);
    }
    
    pub fn helper(k: usize, n: usize, s: &String, mut cache: &mut HashMap<String, bool>, wordSet: &HashSet<String>) -> bool {
        if k == n {
            return true;
        }
        if cache.contains_key(&s[k..].to_string()) {
            //return cache[s.substr(k)];
            return *cache.get(&s[k..].to_string()).unwrap();
        }
        for i in (k + 1)..(n + 1) {
            if wordSet.contains(&s[k..i].to_string()) && Solution::helper(i, n, s, cache, wordSet) {
                cache.insert(s[k..i].to_string(), true);
                return true;
            }
        }
        cache.insert(s[k..].to_string(), false);
        return false;
    }
}
//Given a pattern and a string str, find if str follows the same pattern.
//
//Here follow means a full match, such that there is a bijection between a letter in pattern and a non-empty word in str.
//
//Example 1:
//
//Input: pattern = "abba", str = "dog cat cat dog"
//Output: true
//Example 2:
//
//Input:pattern = "abba", str = "dog cat cat fish"
//Output: false
//Example 3:
//
//Input: pattern = "aaaa", str = "dog cat cat dog"
//Output: false
//Example 4:
//
//Input: pattern = "abba", str = "dog dog dog dog"
//Output: false
//Notes:
//You may assume pattern contains only lowercase letters, and str contains lowercase letters that may be separated by a single space.

use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, str: String) -> bool {
        let mut string: Vec<String> = str.split_whitespace().map(|s| s.to_string()).collect();
        let mut pattern: Vec<char> = pattern.chars().collect();
        if string.len() != pattern.len() {
            return false;
        }
        let mut w_map: HashMap<String, char> = HashMap::new();
        let mut c_map: HashMap<char, String> = HashMap::new();
        let mut c: char = 'a';
        let mut w: String = String::new();
        for i in 0..string.len() {
            c = pattern[i];
            w = string[i].clone();
            if c_map.contains_key(&c) {
                if w != c_map[&c] {
                    return false;
                }
            } else {
                if w_map.contains_key(&w) {
                    return false;
                }
                w_map.insert(w.clone(), c);
                c_map.insert(c, w);
            }
        }
        return true;
    }
}
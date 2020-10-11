//Given a string s, remove duplicate letters so that every letter appears once and only once. You must make sure your result is the smallest in lexicographical order among all possible results.
//
//Note: This question is the same as 1081: https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/
//
// 
//
//Example 1:
//
//Input: s = "bcabc"
//Output: "abc"
//Example 2:
//
//Input: s = "cbacdcbc"
//Output: "acdb"
// 
//
//Constraints:
//
//1 <= s.length <= 104
//s consists of lowercase English letters.
//   Hide Hint #1  
//Greedily try to add one missing character. How to check if adding some character will not cause problems ? Use bit-masks to check whether you will be able to complete the sub-sequence if you add the character at some index i.

use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut last_occ: HashMap<char, usize> = HashMap::new();
        let mut s: Vec<char> = s.chars().collect();
        for i in 0..s.len() {
            last_occ.insert(s[i], i);
        }
        let mut stack: VecDeque<char> = VecDeque::new();
        let mut visited: HashSet<char> = HashSet::new();
        let mut symbol: &char = &' ';
        for i in 0..s.len() {
            symbol = &s[i];
            if(visited.contains(symbol)) {
                continue;
            } else {
                while(!stack.is_empty() && symbol < stack.back().unwrap() && last_occ[stack.back().unwrap()] > i) {
                    visited.remove(&stack.pop_back().unwrap());
                }
                stack.push_back(*symbol);
                visited.insert(*symbol);
            }
        }
        //let mut ans: String = String::new();
        //for c in stack {
        //    ans.push(c);
        //}
        //return ans;
        return stack.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("");
    }
}
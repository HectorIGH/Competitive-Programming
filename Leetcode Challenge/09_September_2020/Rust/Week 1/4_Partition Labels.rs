//A string S of lowercase English letters is given. We want to partition this string into as many parts as possible so that each letter appears in at most one part, and return a list of integers representing the size of these parts.
//
// 
//
//Example 1:
//
//Input: S = "ababcbacadefegdehijhklij"
//Output: [9,7,8]
//Explanation:
//The partition is "ababcbaca", "defegde", "hijhklij".
//This is a partition so that each letter appears in at most one part.
//A partition like "ababcbacadefegde", "hijhklij" is incorrect, because it splits S into less parts.
// 
//
//Note:
//
//S will have length in range [1, 500].
//S will consist of lowercase English letters ('a' to 'z') only.
// 
//
//   Hide Hint #1  
//Try to greedily choose the smallest partition that includes the first letter. If you have something like "abaccbdeffed", then you might need to add b. You can use an map like "last['b'] = 5" to help you expand the width of your partition.

use std::cmp::max;
use std::collections::HashMap;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last: HashMap<char, i32> = HashMap::new();
        let mut S:Vec<char> = s.chars().collect();
        let mut n: usize = S.len();
        let mut j: i32 = 0;
        let mut anchor: i32 = 0;
        let mut ans: Vec<i32> = Vec::new();
        
        for i in 0..n {
            last.insert(S[i], i as i32);
        }
        for i in 0..(n as i32) {
            j = max(j, last[&S[i as usize]]);
            if i == j {
                ans.push(i - anchor + 1);
                anchor = i + 1;
            }
        }
        ans
    }
}
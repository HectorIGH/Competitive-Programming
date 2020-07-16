//Given a string s and a string t, check if s is subsequence of t.
//
//A subsequence of a string is a new string which is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (ie, "ace" is a subsequence of "abcde" while "aec" is not).
//
//Follow up:
//If there are lots of incoming S, say S1, S2, ... , Sk where k >= 1B, and you want to check one by one to see if T has its subsequence. In this scenario, how would you change your code?
//
//Credits:
//Special thanks to @pbrother for adding this problem and creating all test cases.
//
// 
//
//Example 1:
//
//Input: s = "abc", t = "ahbgdc"
//Output: true
//Example 2:
//
//Input: s = "axc", t = "ahbgdc"
//Output: false
// 
//
//Constraints:
//
//0 <= s.length <= 100
//0 <= t.length <= 10^4
//Both strings consists only of lowercase characters.

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        // DP
        let mut t = t.chars().collect::<Vec<_>>();
        let mut s = s.chars().collect::<Vec<_>>();
        t.insert(0, '#');
        s.insert(0, '#');
        let n = t.len();
        let m = s.len();
        let mut dp = vec![vec![0; m]; n];
        dp[0][0] = 1;
        for i in 1..n {
            dp[i][0] = 1;
            for j in 1..m {
                if s[j] == t[i] {
                    dp[i][j] = dp[i-1][j-1];
                } else {
                    dp[i][j] = dp[i-1][j];
                }
            }
        }
        dp[n-1][m-1] != 0
        
        // Two pointer
        /*
        let t = t.chars().collect::<Vec<_>>();
        let s = s.chars().collect::<Vec<_>>();
        let mut sp:usize = 0;
        let mut tp:usize = 0;
        while sp < s.len() && tp < t.len() {
            //sp += if s.chars().nth(sp) == t.chars().nth(tp) {1} else {0}; // To acces nth element without converting to vector. Way slower.
            sp += if s[sp] == t[tp] {1} else {0};
            tp += 1;
        }
        sp == s.len()
        */
    }
}
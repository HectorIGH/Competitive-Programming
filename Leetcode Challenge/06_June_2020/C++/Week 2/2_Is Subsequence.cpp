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

class Solution {
public:
    bool isSubsequence(string s, string t) {
        s = '#' + s;
        t = '#' + t;
        int m = s.size(), n = t.size();
        vector<vector<int>> dp(n, vector<int>(m, 0));
        dp[0][0] = 1;
        for(int i = 1; i < n; i++){
            dp[i][0] = 1;
            for(int j = 1; j < m; j++){
                if(s[j] == t[i]){
                    dp[i][j] = dp[i-1][j-1];
                } else{
                    dp[i][j] = dp[i-1][j];
                }
            }
        }
        return dp[n-1][m-1] != 0;
        
        // Two pointers
        /*
        int sp = 0, tp = 0;
        while ((sp < s.size()) && (tp < t.size())){
            sp += s[sp] == t[tp] ? 1 : 0;
            tp++;
        }
        return sp == s.size();
        */
    }
};
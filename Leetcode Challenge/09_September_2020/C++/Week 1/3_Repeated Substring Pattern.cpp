//Given a non-empty string check if it can be constructed by taking a substring of it and appending multiple copies of the substring together. You may assume the given string consists of lowercase English letters only and its length will not exceed 10000.
//
// 
//
//Example 1:
//
//Input: "abab"
//Output: True
//Explanation: It's the substring "ab" twice.
//Example 2:
//
//Input: "aba"
//Output: False
//Example 3:
//
//Input: "abcabcabcabc"
//Output: True
//Explanation: It's the substring "abc" four times. (And the substring "abcabc" twice.)

class Solution {
public:
    bool repeatedSubstringPattern(string s) {
        int n = s.size();
        string test;
        string sub;
        for(int i = 1; i < n / 2 + 1; i++) {
            if(n % i == 0) {
                test = "";
                sub = s.substr(0, i);
                for(int j = 0; j < n / i; j++) {
                    test += sub;
                }
                if(s == test) {
                    return true;
                }
            } 
        }
        return false;
    }
};
//Given a string, determine if it is a palindrome, considering only alphanumeric characters and ignoring cases.
//
//Note: For the purpose of this problem, we define empty string as valid palindrome.
//
//Example 1:
//
//Input: "A man, a plan, a canal: Panama"
//Output: true
//Example 2:
//
//Input: "race a car"
//Output: false
// 
//
//Constraints:
//
//s consists only of printable ASCII characters.

class Solution {
public:
    bool isPalindrome(string s) {
        int l = 0, r = s.size() - 1;
        while(l <= r) {
            while(!isalnum(s[l]) && l < r) {
                l++;
            }
            while(!isalnum(s[r]) && l < r) {
                r--;
            }
            if(s[l] == s[r] || tolower(s[l]) == tolower(s[r])) {
                l++;
                r--;
            } else {
                return false;
            }
        }
        return true;
    }
};
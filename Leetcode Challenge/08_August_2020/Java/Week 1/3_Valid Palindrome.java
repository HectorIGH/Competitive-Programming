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
    public boolean isPalindrome(String s) {
        int l = 0, r = s.length() - 1;
        while(l <= r) {
            while(!Character.isLetterOrDigit(s.charAt(l)) && l < r) {
                l++;
            }
            while(!Character.isLetterOrDigit(s.charAt(r)) && l < r) {
                r--;
            }
            if(s.charAt(l) == s.charAt(r) || Character.toLowerCase(s.charAt(l)) == Character.toLowerCase(s.charAt(r))) {
                l++;
                r--;
            } else {
                return false;
            }
        }
        return true;
    }
}
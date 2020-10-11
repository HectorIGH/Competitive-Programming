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

class Solution {
    public String removeDuplicateLetters(String s) {
        HashMap<Character, Integer> last_occ = new HashMap<>();
        for(int i = 0; i < s.length(); i++) {
            last_occ.put(s.charAt(i), i);
        }
        Stack<Character> stack = new Stack<>();
        HashSet<Character> visited = new HashSet<>();
        Character symbol = ' ';
        for(int i = 0; i < s.length(); i ++) {
            symbol = s.charAt(i);
            if(visited.contains(symbol)) {
                continue;
            } else {
                while(!stack.empty() && symbol < stack.peek() && last_occ.get(stack.peek()) > i) {
                    visited.remove(stack.pop());
                }
                stack.push(symbol);
                visited.add(symbol);
            }
        }
        String ans = "";
        for(char c : stack) {
            ans += Character.toString(c);
        }
        return ans;
    }
}
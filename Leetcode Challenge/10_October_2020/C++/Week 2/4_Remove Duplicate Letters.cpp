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
public:
    string removeDuplicateLetters(string s) {
        unordered_map<char, int> last_occ;
        for(int i = 0; i < s.size(); i++) {
            last_occ[s[i]] = i;
        }
        vector<char> stack;
        unordered_set<char> visited;
        char symbol = ' ';
        for(int i = 0; i < s.size(); i ++) {
            symbol = s[i];
            if(visited.count(symbol)) {
                continue;
            } else {
                while(!stack.empty() && symbol < stack.back() && last_occ[stack.back()] > i) {
                    visited.erase(stack.back());
                    stack.pop_back();
                }
                stack.push_back(symbol);
                visited.insert(symbol);
            }
        }
        string ans = "";
        for(char c : stack) {
            ans += c;
        }
        return ans;
    }
};
//Given a non-empty string s and a dictionary wordDict containing a list of non-empty words, add spaces in s to construct a sentence where each word is a valid dictionary word. Return all such possible sentences.
//
//Note:
//
//The same word in the dictionary may be reused multiple times in the segmentation.
//You may assume the dictionary does not contain duplicate words.
//Example 1:
//
//Input:
//s = "catsanddog"
//wordDict = ["cat", "cats", "and", "sand", "dog"]
//Output:
//[
//  "cats and dog",
//  "cat sand dog"
//]
//Example 2:
//
//Input:
//s = "pineapplepenapple"
//wordDict = ["apple", "pen", "applepen", "pine", "pineapple"]
//Output:
//[
//  "pine apple pen apple",
//  "pineapple pen apple",
//  "pine applepen apple"
//]
//Explanation: Note that you are allowed to reuse a dictionary word.
//Example 3:
//
//Input:
//s = "catsandog"
//wordDict = ["cats", "dog", "sand", "and", "cat"]
//Output:
//[]

class Solution {
public:
    vector<string> wordBreak(string s, vector<string>& wordDict) {
        set<string> dic(wordDict.begin(), wordDict.end());
        unordered_map<int, vector<string>> memo;
        return dfs(s, dic, 0, memo);
    }
    vector<string> dfs(string s, set<string>& dic, int idx, unordered_map<int, vector<string>>& memo) {
        if(memo.count(idx)) {
            return memo[idx];
        }
        vector<string> res;
        for(int i = idx; i < s.length(); i++) {
            string cur = s.substr(idx, i - idx + 1);
            if(dic.count(cur)) {
                if(i == s.size() - 1) {
                    res.push_back(cur);
                }
                vector<string> temp = dfs(s, dic, i + 1, memo);
                for(string e : temp) {
                    res.push_back(cur + " " + e);
                }
            }
        }
        memo[idx] = res;
        return memo[idx];
    }
};
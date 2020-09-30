//Given a non-empty string s and a dictionary wordDict containing a list of non-empty words, determine if s can be segmented into a space-separated sequence of one or more dictionary words.
//
//Note:
//
//The same word in the dictionary may be reused multiple times in the segmentation.
//You may assume the dictionary does not contain duplicate words.
//Example 1:
//
//Input: s = "leetcode", wordDict = ["leet", "code"]
//Output: true
//Explanation: Return true because "leetcode" can be segmented as "leet code".
//Example 2:
//
//Input: s = "applepenapple", wordDict = ["apple", "pen"]
//Output: true
//Explanation: Return true because "applepenapple" can be segmented as "apple pen apple".
//             Note that you are allowed to reuse a dictionary word.
//Example 3:
//
//Input: s = "catsandog", wordDict = ["cats", "dog", "sand", "and", "cat"]
//Output: false

class Solution {
public:
    
    bool wordBreak(string s, vector<string>& wordDict) {
        unordered_map<string, bool> cache;
        unordered_set<string> wordSet(wordDict.begin(), wordDict.end());
        int n = s.size();
        return helper(0, n, s, cache, wordSet);
    }
    
    bool helper(int k, int n, string &s, unordered_map<string, bool> &cache, unordered_set<string> &wordSet) {
        if(k == n) {
            return true;
        }
        if(cache.count(s.substr(k))) {
            return cache[s.substr(k)];
        }
        for(int i = k + 1; i < n + 1; i++) {
            if(wordSet.count(s.substr(k, i - k)) && helper(i, n, s, cache, wordSet)) {
                cache[s.substr(k, i - k)] = true;
                return true;
            }
        }
        cache[s.substr(k)] = false;
        return false;
    }
};
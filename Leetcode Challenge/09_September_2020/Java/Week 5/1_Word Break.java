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
    HashMap<String, Boolean> cache = new HashMap<>();
    HashSet<String> wordSet = new HashSet<>();
    public boolean wordBreak(String s, List<String> wordDict) {
        for(String word : wordDict) {
            wordSet.add(word);
        }
        int n = s.length();
        return helper(0, n, s);
    }
    public boolean helper(int k, int n, String s) {
        if(k == n) {
            return true;
        }
        if(cache.containsKey(s.substring(k))) {
            return cache.get(s.substring(k));
        }
        for(int i = k + 1; i < n + 1; i++) {
            if(wordSet.contains(s.substring(k, i)) && helper(i, n, s)) {
                cache.put(s.substring(k, i), true);
                return true;
            }
        }
        cache.put(s.substring(k), false);
        return false;
    }
}
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
    public List<String> wordBreak(String s, List<String> wordDict) {
        Set<String> dic = new HashSet<>(wordDict);
        HashMap<Integer, List<String>> memo = new HashMap<>();
        return dfs(s, dic, 0, memo);
    }

    public List<String> dfs(String s, Set<String> dic, int idx, HashMap<Integer, List<String>> memo) {
        if(memo.containsKey(idx)) {
            return memo.get(idx);
        }
        List<String> res = new ArrayList<>();
        for(int i = idx; i < s.length(); i++) {
            String cur = s.substring(idx, i - idx + 1 + idx);
            if(dic.contains(cur)) {
                if(i == s.length() - 1) {
                    res.add(cur);
                }
                List<String> temp = dfs(s, dic, i + 1, memo);
                for(String e : temp) {
                    res.add(cur + " " + e);
                }
            }
        }
        memo.put(idx, res);
        return memo.get(idx);
    }
}
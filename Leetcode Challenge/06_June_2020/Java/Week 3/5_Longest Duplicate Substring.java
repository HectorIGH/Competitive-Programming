//Given a string S, consider all duplicated substrings: (contiguous) substrings of S that occur 2 or more times.  (The occurrences may overlap.)
//
//Return any duplicated substring that has the longest possible length.  (If S does not have a duplicated substring, the answer is "".)
//
// 
//
//Example 1:
//
//Input: "banana"
//Output: "ana"
//Example 2:
//
//Input: "abcd"
//Output: ""
// 
//
//Note:
//
//2 <= S.length <= 10^5
//S consists of lowercase English letters.
//   Hide Hint #1  
//Binary search for the length of the answer. (If there's an answer of length 10, then there are answers of length 9, 8, 7, ...)
//   Hide Hint #2  
//To check whether an answer of length K exists, we can use Rabin-Karp 's algorithm.


class Solution {
    public String RabinKarp(String text, int M, int q) {
        if(M == 0) {
            return "true";
        }
        long h = (1<<(8*M-8)) % q, t = 0, d = 256;
        HashMap<Long, List<Integer>> dic = new HashMap<>();
        
        for(int i = 0; i < M; i++) {
            t = (d * t + (int)text.charAt(i)) % q;
        }
        dic.putIfAbsent(t, new ArrayList<>());
        dic.get(t).add(0);
        
        for(int i = 0; i < text.length() - M; i++) {
            t = (d * (t - ((int)text.charAt(i) * h)) + (int)text.charAt(i + M)) % q;
            if(dic.containsKey(t)) {
                for(int j : dic.get(t)) {
                    if(text.substring(i+1, i+M+1).equals(text.substring(j, j + M))) {
                        return text.substring(j, j + M);
                    }
                }
            }
            dic.putIfAbsent(t, new ArrayList<>());
            dic.get(t).add(i + 1);
        }
        return "";
    }
    public String longestDupSubstring(String S) {
        int l = 0, r = S.length(), mid = 0, q = (1<<31) + 10;
        String ans = "", candidate = "";
        while(l + 1 < r) {
            mid = l + (int)((r - l) / 2);
            candidate = RabinKarp(S, mid, q);
            if(!candidate.equals("")) {
                l = mid;
                ans = candidate;
            } else {
                r = mid;
            }
        }
        return ans;
    }
}
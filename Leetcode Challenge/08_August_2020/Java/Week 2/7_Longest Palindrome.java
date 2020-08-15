//Given a string which consists of lowercase or uppercase letters, find the length of the longest palindromes that can be built with those letters.
//
//This is case sensitive, for example "Aa" is not considered a palindrome here.
//
//Note:
//Assume the length of given string will not exceed 1,010.
//
//Example:
//
//Input:
//"abccccdd"
//
//Output:
//7
//
//Explanation:
//One longest palindrome that can be built is "dccaccd", whose length is 7.

class Solution {
    public int longestPalindrome(String s) {
        //HashMap<Character, Integer> f = new HashMap<>();
        int[] g = new int[58];
        boolean odd = false;
        int ans = 0;
        for(int i = 0; i < s.length(); i++) {
            Character l = s.charAt(i);
            //f.put(l, f.getOrDefault(l, 0) + 1);
            g[(int)l - 65]++;
        }
            
        for(int l : g) {
        //for(Map.Entry<Character, Integer> entry : f.entrySet()) {
            //int l = entry.getValue();
            if((l & 1) != 0) {
                if(!odd) {
                    ans += l;
                    odd = true;
                    continue;
                } else {
                    odd = true;
                }
            }
            ans += ((l >> 1) << 1);
        }
        return ans;
    }
}
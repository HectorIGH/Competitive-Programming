//Given two strings s and t which consist of only lowercase letters.
//
//String t is generated by random shuffling string s and then add one more letter at a random position.
//
//Find the letter that was added in t.
//
//Example:
//
//Input:
//s = "abcd"
//t = "abcde"
//
//Output:
//e
//
//Explanation:
//'e' is the letter that was added.

class Solution {
    public char findTheDifference(String s, String t) {
        /*
        int[] sc = new int[26];
        int[] tc = new int[26];
        char l = 'Z';
        for(int i = 0; i < s.length(); i++) {
            l = s.charAt(i);
            sc[l - 'a'] += 1;
        }
        for(int i = 0; i < t.length(); i++) {
            l = t.charAt(i);
            tc[l - 'a'] += 1;
        }
        for(int i = 0; i < 26; i++) {
            if(sc[i] != tc[i]) {
                return (char)(i + 'a');
            }
        }
        return l;
        */
        char l = 'Z';
        int x = 0;
        for(int i = 0; i < s.length(); i++) {
            l = s.charAt(i);
            x ^= (l - 'a');
        }
        for(int i = 0; i < t.length(); i++) {
            l = t.charAt(i);
            x ^= (l - 'a');
        }
        return (char)(x + 'a');
        
    }
}
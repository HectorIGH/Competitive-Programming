//Write a function that reverses a string. The input string is given as an array of characters char[].
//
//Do not allocate extra space for another array, you must do this by modifying the input array in-place with O(1) extra memory.
//
//You may assume all the characters consist of printable ascii characters.
//
// 
//
//Example 1:
//
//Input: ["h","e","l","l","o"]
//Output: ["o","l","l","e","h"]
//Example 2:
//
//Input: ["H","a","n","n","a","h"]
//Output: ["h","a","n","n","a","H"]
//   Hide Hint #1  
//The entire logic for reversing a string is based on using the opposite directional two-pointer approach!

class Solution {
    public void reverseString(char[] s) {
        int n = s.length;
        for(int i = 0; i < n/2; i++){
            char a = s[i];
            s[i] = s[n - 1 - i];
            s[n - 1 - i] = a;
        }
    }
}
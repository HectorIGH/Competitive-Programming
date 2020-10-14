//Given two strings A and B of lowercase letters, return true if you can swap two letters in A so the result is equal to B, otherwise, return false.
//
//Swapping letters is defined as taking two indices i and j (0-indexed) such that i != j and swapping the characters at A[i] and A[j]. For example, swapping at indices 0 and 2 in "abcd" results in "cbad".
//
// 
//
//Example 1:
//
//Input: A = "ab", B = "ba"
//Output: true
//Explanation: You can swap A[0] = 'a' and A[1] = 'b' to get "ba", which is equal to B.
//Example 2:
//
//Input: A = "ab", B = "ab"
//Output: false
//Explanation: The only letters you can swap are A[0] = 'a' and A[1] = 'b', which results in "ba" != B.
//Example 3:
//
//Input: A = "aa", B = "aa"
//Output: true
//Explanation: You can swap A[0] = 'a' and A[1] = 'a' to get "aa", which is equal to B.
//Example 4:
//
//Input: A = "aaaaaaabc", B = "aaaaaaacb"
//Output: true
//Example 5:
//
//Input: A = "", B = "aa"
//Output: false
// 
//
//Constraints:
//
//0 <= A.length <= 20000
//0 <= B.length <= 20000
//A and B consist of lowercase letters.

use std::collections::HashSet;

impl Solution {
    pub fn buddy_strings(a: String, b: String) -> bool {
        if(a.len() != b.len()) {
            return false;
        }
        if a == b {
            let mut a: Vec<char> = a.chars().collect();
            let mut b: Vec<char> = b.chars().collect();
            let mut seen: HashSet<char> = HashSet::new();
            let mut aa: char = ' ';
            for i in 0..a.len() {
                aa = a[i];
                if seen.contains(&aa) {
                    return true;
                }
                seen.insert(aa);
            }
            return false;
        } else {
            let mut first: usize = 20000 + 1;
            let mut second: usize = 20000 + 1;
            let mut a: Vec<char> = a.chars().collect();
            let mut b: Vec<char> = b.chars().collect();
            for i in 0..a.len() {
                if a[i] != b[i] {
                    if first == 20000 + 1 {
                        first = i;
                    } else if second == 20000 + 1 {
                        second = i;
                    } else {
                        return false;
                    }
                }
            }
            return second != 20000 + 1 && a[first] == b[second] && a[second] == b[first];
        }
    }
}
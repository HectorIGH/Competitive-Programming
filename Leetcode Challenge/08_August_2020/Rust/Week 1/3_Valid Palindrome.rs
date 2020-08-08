//Given a string, determine if it is a palindrome, considering only alphanumeric characters and ignoring cases.
//
//Note: For the purpose of this problem, we define empty string as valid palindrome.
//
//Example 1:
//
//Input: "A man, a plan, a canal: Panama"
//Output: true
//Example 2:
//
//Input: "race a car"
//Output: false
// 
//
//Constraints:
//
//s consists only of printable ASCII characters.

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s : Vec<char> = s.chars().collect();
        let mut l : i32 = 0;
        let mut r : i32 = (s.len() - 1) as i32;
        while l <= r {
            while !s[l as usize].is_alphanumeric() && l < r {
                l += 1;
            }
            while !s[r as usize].is_alphanumeric() && l < r {
                r -= 1;
            }
            let sl : Vec<char> = s[l as usize].to_lowercase()
            .collect();
            let sr : Vec<char> = s[r as usize].to_lowercase()
            .collect();
            if s[l as usize] == s[r as usize] || sl[0] == sr[0] {
                l += 1;
                r -= 1;
            } else {
                return false;
            }
        }
        true
    }
}
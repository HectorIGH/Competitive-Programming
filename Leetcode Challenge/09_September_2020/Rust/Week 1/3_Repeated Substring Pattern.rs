//Given a non-empty string check if it can be constructed by taking a substring of it and appending multiple copies of the substring together. You may assume the given string consists of lowercase English letters only and its length will not exceed 10000.
//
// 
//
//Example 1:
//
//Input: "abab"
//Output: True
//Explanation: It's the substring "ab" twice.
//Example 2:
//
//Input: "aba"
//Output: False
//Example 3:
//
//Input: "abcabcabcabc"
//Output: True
//Explanation: It's the substring "abc" four times. (And the substring "abcabc" twice.)

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let n: usize = s.len();
        let mut test: String = String::new();
        let mut sub: String = String::new();
        for i in 1..(n / 2 + 1) {
            if n % i == 0 {
                test = "".to_string();
                sub = s.chars().take(i).collect();
                for j in 0..(n / i) {
                    //test.push_str(&sub);
                    test += &sub;
                }
                if s == test {
                    return true;
                }
            } 
        }
        return false;
    }
}
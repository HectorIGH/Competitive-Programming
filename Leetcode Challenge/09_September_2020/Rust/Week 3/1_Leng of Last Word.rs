//Given a string s consists of upper/lower-case alphabets and empty space characters ' ', return the length of last word (last word means the last appearing word if we loop from left to right) in the string.
//
//If the last word does not exist, return 0.
//
//Note: A word is defined as a maximal substring consisting of non-space characters only.
//
//Example:
//
//Input: "Hello World"
//Output: 5

impl Solution {
    pub fn length_of_last_word(ss: String) -> i32 {
        if ss.is_empty() {
            return 0;
        }
        let mut s: Vec<char> = ss.chars().collect();
        let mut n: i32 = s.len() as i32 - 1;
        let mut i: i32 = 0;
        while n >= 0 && s[n as usize] == ' ' {
            n -= 1;
        }
        while n >= 0 && s[n as usize] != ' ' {
            i += 1;
            n -= 1;
        }
        return i;
    }
}
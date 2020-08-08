//Given a word, you need to judge whether the usage of capitals in it is right or not.
//
//We define the usage of capitals in a word to be right when one of the following cases holds:
//
//All letters in this word are capitals, like "USA".
//All letters in this word are not capitals, like "leetcode".
//Only the first letter in this word is capital, like "Google".
//Otherwise, we define that this word doesn't use capitals in a right way.
// 
//
//Example 1:
//
//Input: "USA"
//Output: True
// 
//
//Example 2:
//
//Input: "FlaG"
//Output: False
// 
//
//Note: The input will be a non-empty word consisting of uppercase and lowercase latin letters.

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let l : Vec<char> = word.chars().collect();
        if l[0].is_lowercase() {
            return l.iter().all(|&c| c.is_lowercase());
        }
        if l.len() > 1 {
            let b = l[1].is_lowercase();
            return l.iter().skip(1).all(|&c| c.is_lowercase() == b);
        }
        true
    }
}
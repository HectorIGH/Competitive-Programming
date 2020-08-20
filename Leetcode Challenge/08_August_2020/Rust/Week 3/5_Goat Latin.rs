//A sentence S is given, composed of words separated by spaces. Each word consists of lowercase and uppercase letters only.
//
//We would like to convert the sentence to "Goat Latin" (a made-up language similar to Pig Latin.)
//
//The rules of Goat Latin are as follows:
//
//If a word begins with a vowel (a, e, i, o, or u), append "ma" to the end of the word.
//For example, the word 'apple' becomes 'applema'.
// 
//If a word begins with a consonant (i.e. not a vowel), remove the first letter and append it to the end, then add "ma".
//For example, the word "goat" becomes "oatgma".
// 
//Add one letter 'a' to the end of each word per its word index in the sentence, starting with 1.
//For example, the first word gets "a" added to the end, the second word gets "aa" added to the end and so on.
//Return the final sentence representing the conversion from S to Goat Latin. 
//
// 
//
//Example 1:
//
//Input: "I speak Goat Latin"
//Output: "Imaa peaksmaaa oatGmaaaa atinLmaaaaa"
//Example 2:
//
//Input: "The quick brown fox jumped over the lazy dog"
//Output: "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa"
// 
//
//Notes:
//
//S contains only uppercase, lowercase and spaces. Exactly one space between each word.
//1 <= S.length <= 150.

use std::collections::HashSet;

impl Solution {
    pub fn to_goat_latin(s: String) -> String {
        let v: HashSet<char> = "aeiouAEIOU".chars().collect();
        
        let mut S: String = String::new();
        let mut suff: String = String::from("a");
        
        for w in s.split_whitespace() {
            let mut cu: String = String::new();
            if !v.contains(&w.chars().nth(0).expect("")) {
                cu = (&w[1..]).to_string();
                cu.push(w.chars().nth(0).expect(""));
            } else {
                cu = w.to_string();
            }
            S.push_str(&cu);
            S.push_str("ma");
            S.push_str(&suff);
            S.push(' ');
            suff.push('a');
        }
        S.pop();        
        S
    }
}
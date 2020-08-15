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

use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut f: HashMap<char, i32> = HashMap::new();
        //let mut g: Vec<i32> = vec![0; 58];
        let mut odd: bool = false;
        let mut ans: i32 = 0;
        
        //for &l in s.as_bytes() {
        for l in s.chars() {
            let mut count = f.entry(l).or_insert(0);
            *count += 1;
            //g[(l - b'A') as usize] += 1;
        }
            
        //for l in g {
        for (_, l) in f.iter() {
            if l & 1 != 0 {
                if !odd {
                    ans += l;
                    odd = true;
                    continue;
                } else {
                    odd = true;
                }
            }
            ans += (l >> 1) << 1;
        }
        ans
    }
}
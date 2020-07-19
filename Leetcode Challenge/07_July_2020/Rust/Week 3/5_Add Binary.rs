//Given two binary strings, return their sum (also a binary string).
//
//The input strings are both non-empty and contains only characters 1 or 0.
//
//Example 1:
//
//Input: a = "11", b = "1"
//Output: "100"
//Example 2:
//
//Input: a = "1010", b = "1011"
//Output: "10101"
// 
//
//Constraints:
//
//Each string consists only of '0' or '1' characters.
//1 <= a.length, b.length <= 10^4
//Each string is either "0" or doesn't contain any leading zero.

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut A : Vec<char> = a.chars().collect();
        let mut B : Vec<char> = b.chars().collect();
        let mut i = (A.len() - 1) as i32;
        let mut j = (B.len() - 1) as i32;
        
        let mut carry = 0;
        let mut ans : Vec<char> = Vec::new();
        
        while i >= 0 || j >= 0 || carry == 1 {
            let mut sum : u8 = carry;
            sum += if i >= 0 {A[i as usize] as u8 - 48} else {0};
            sum += if j >= 0 {B[j as usize] as u8 - 48} else {0};
            ans.push(if (sum & 1) == 1 {'1'} else {'0'});
            carry = sum >> 1;
            i -= 1;
            j -= 1;
        }
        return ans.iter().rev().collect();
    }
}
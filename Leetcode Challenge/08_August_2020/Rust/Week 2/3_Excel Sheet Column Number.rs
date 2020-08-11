//Given a column title as appear in an Excel sheet, return its corresponding column number.
//
//For example:
//
//    A -> 1
//    B -> 2
//    C -> 3
//    ...
//    Z -> 26
//    AA -> 27
//    AB -> 28 
//    ...
//Example 1:
//
//Input: "A"
//Output: 1
//Example 2:
//
//Input: "AB"
//Output: 28
//Example 3:
//
//Input: "ZY"
//Output: 701
// 
//
//Constraints:
//
//1 <= s.length <= 7
//s consists only of uppercase English letters.
//s is between "A" and "FXSHRXW".

impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        //let mut p: u32 = s.len() as u32 - 1;
        let mut p: i32 = 1;
        let mut ans: i32 = 0;
        /*
        for c in s.as_bytes() {
            ans += (c - 64) as i32 * i32::pow(26, p);
            p -= 1;
        }
        */
        for c in s.as_bytes().iter().rev() {
            ans += (c - 64) as i32 * p;
            p *= 26;
        }
        ans
    }
}
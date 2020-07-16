//Given an integer, write a function to determine if it is a power of two.
//
//Example 1:
//
//Input: 1
//Output: true 
//Explanation: 20 = 1
//Example 2:
//
//Input: 16
//Output: true
//Explanation: 24 = 16
//Example 3:
//
//Input: 218
//Output: false

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n == 0 {
            return false;
        }
        let mut ones:i32 = 0;
        let mut nn = n;
        while nn != 0 {
            ones += nn & 1;
            if ones > 1 {
                return false;
            }
            nn >>= 1;
        }
        return true;
    }
}
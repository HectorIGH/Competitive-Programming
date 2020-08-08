//Given an integer (signed 32 bits), write a function to check whether it is a power of 4.
//
//Example 1:
//
//Input: 16
//Output: true
//Example 2:
//
//Input: 5
//Output: false
//Follow up: Could you solve it without loops/recursion?

impl Solution {
    pub fn is_power_of_four(mut num: i32) -> bool {
        /*
        let mut i: u8 = 0;
        while num != 0 {
            if num & 1 == 1 {
                break;
            } else {}
            i += 1;
            num >>= 1;
        }
        num == 1 && (i & 1) == 0
        */
        /*
        num > 0 && num == i32::pow(4, (num as f64).log(4.0) as u32)
        */
        num > 0 && num & (num - 1) == 0 && num & 0b1010101010101010101010101010101 != 0
    }
}
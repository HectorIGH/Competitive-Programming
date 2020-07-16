//Given a non-empty array of integers, every element appears three times except for one, which appears exactly once. Find that single one.
//
//Note:
//
//Your algorithm should have a linear runtime complexity. Could you implement it without using extra memory?
//
//Example 1:
//
//Input: [2,2,3,2]
//Output: 3
//Example 2:
//
//Input: [0,1,0,1,0,1,99]
//Output: 99

use std:: collections::HashMap;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        // Bitwise O(1) space
        /*
        let mut ones : i32 = 0;
        let mut twos : i32 = 0;
        let mut not_threes : i32 = 0;
        for n in &nums {
            twos |= ones & n;
            ones ^= n;
            not_threes = !(ones & twos);
            ones &= not_threes;
            twos &= not_threes;
        }
        ones
        */
        // HashMap space O(n)
        let mut freq : HashMap<i32, i32> = HashMap::new();
        for n in &nums {
            freq.insert(*n, 1 + if freq.contains_key(&n) {freq[n]} else {0});
        }
        for n in &nums {
            if freq[n] == 1 {
                return *n;
            }
        }
        return 0;
    }
}
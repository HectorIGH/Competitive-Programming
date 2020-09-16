//Given a non-empty array of numbers, a0, a1, a2, … , an-1, where 0 ≤ ai < 231.
//
//Find the maximum result of ai XOR aj, where 0 ≤ i, j < n.
//
//Could you do this in O(n) runtime?
//
//Example:
//
//Input: [3, 10, 5, 25, 2, 8]
//
//Output: 28
//
//Explanation: The maximum result is 5 ^ 25 = 28.

use std::collections::HashSet;

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut ans: i64 = 0;
        let mut mask: i64 = 0;
        let mut start: i64 = 0;
        let mut found: HashSet<i64> = HashSet::new();

        for i in (0..32).rev() {
            mask |= 1 << i;
            found.clear();
            for num in &nums {
                found.insert(*num as i64 & mask);
            }
            start = ans | 1 << i;
            for pref in found.iter() {
                if found.contains(&(start ^ pref)) {
                    ans = start;
                    break;
                }
            }
        }
        return ans as i32;
    }
}
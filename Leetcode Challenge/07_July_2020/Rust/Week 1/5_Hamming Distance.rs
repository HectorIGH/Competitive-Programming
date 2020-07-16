//The Hamming distance between two integers is the number of positions at which the corresponding bits are different.
//
//Given two integers x and y, calculate the Hamming distance.
//
//Note:
//0 ≤ x, y < 231.
//
//Example:
//
//Input: x = 1, y = 4
//
//Output: 2
//
//Explanation:
//1   (0 0 0 1)
//4   (0 1 0 0)
//       ↑   ↑
//
//The above arrows point to positions where the corresponding bits are different.

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut partial = x^y;
        let mut count = 0;
        while partial != 0 {
            /*
            if partial & 1 != 0 {
                count += 1;
            }
            partial >>= 1;
            */
            partial &= partial - 1;
            count += 1;
        }
        count
    }
}
//An integer has sequential digits if and only if each digit in the number is one more than the previous digit.
//
//Return a sorted list of all the integers in the range [low, high] inclusive that have sequential digits.
//
// 
//
//Example 1:
//
//Input: low = 100, high = 300
//Output: [123,234]
//Example 2:
//
//Input: low = 1000, high = 13000
//Output: [1234,2345,3456,4567,5678,6789,12345]
// 
//
//Constraints:
//
//10 <= low <= high <= 10^9
//   Hide Hint #1  
//Generate all numbers with sequential digits and check if they are in the given range.
//   Hide Hint #2  
//Fix the starting digit then do a recursion that tries to append all valid digits.

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut s: String = String::from("123456789");
        let mut sub: String = String::new();
        let mut v: Vec<i32> = Vec::new();
        let mut ans: i32 = 0;
        for i in 0..9 {
            for j in i..9 {
                sub.clear();
                sub = s[i..(j + 1)].to_string();
                ans = sub.parse::<i32>().unwrap();
                if ans >= low && ans <= high {
                    v.push(ans);
                }
            }
        }
        v.sort_unstable();
        return v;
    }
}
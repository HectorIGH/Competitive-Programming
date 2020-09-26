//Given a list of non negative integers, arrange them such that they form the largest number.
//
//Example 1:
//
//Input: [10,2]
//Output: "210"
//Example 2:
//
//Input: [3,30,34,5,9]
//Output: "9534330"
//Note: The result may be very large, so you need to return a string instead of an integer.

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut s: Vec<String> = nums.iter().map(|&n| n.to_string()).collect();
        s.sort_by(|x: &String, y: &String| (y.clone() + x).cmp(&(x.clone() + y)));
        if s[0] == "0" {
            return String::from("0")
        } else {
            s.join("")
        }
    }
}
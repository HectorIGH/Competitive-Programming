//Write a program that outputs the string representation of numbers from 1 to n.
//
//But for multiples of three it should output “Fizz” instead of the number and for the multiples of five output “Buzz”. For numbers which are multiples of both three and five output “FizzBuzz”.
//
//Example:
//
//n = 15,
//
//Return:
//[
//    "1",
//    "2",
//    "Fizz",
//    "4",
//    "Buzz",
//    "Fizz",
//    "7",
//    "8",
//    "Fizz",
//    "Buzz",
//    "11",
//    "Fizz",
//    "13",
//    "14",
//    "FizzBuzz"
//]

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut ans: Vec<String> = Vec::with_capacity(n as usize);
        let mut s: String = String::new();
        for i in 1..(n + 1) {
            s.clear();
            if i % 3 == 0 {
                s.push_str("Fizz");
            }
            if i % 5 == 0 {
                s.push_str("Buzz");
            }
            if s.is_empty() {
                s = i.to_string();
            }
            ans.push(s.clone());
        }
        ans
    }
}
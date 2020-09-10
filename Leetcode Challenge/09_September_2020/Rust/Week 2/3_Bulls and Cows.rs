//You are playing the following Bulls and Cows game with your friend: You write down a number and ask your friend to guess what the number is. Each time your friend makes a guess, you provide a hint that indicates how many digits in said guess match your secret number exactly in both digit and position (called "bulls") and how many digits match the secret number but locate in the wrong position (called "cows"). Your friend will use successive guesses and hints to eventually derive the secret number.
//
//Write a function to return a hint according to the secret number and friend's guess, use A to indicate the bulls and B to indicate the cows. 
//
//Please note that both secret number and friend's guess may contain duplicate digits.
//
//Example 1:
//
//Input: secret = "1807", guess = "7810"
//
//Output: "1A3B"
//
//Explanation: 1 bull and 3 cows. The bull is 8, the cows are 0, 1 and 7.
//Example 2:
//
//Input: secret = "1123", guess = "0111"
//
//Output: "1A1B"
//
//Explanation: The 1st 1 in friend's guess is a bull, the 2nd or 3rd 1 is a cow.
//Note: You may assume that the secret number and your friend's guess only contain digits, and their lengths are always equal.

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut s: Vec<u8> = secret.chars().map(|x| x.to_digit(10).unwrap() as u8).collect();
        let mut g: Vec<u8> = guess.chars().map(|x| x.to_digit(10).unwrap() as u8).collect();
        let mut a: u16 = 0;
        let mut b: u16 = 0;
        let mut left: Vec<u8> = Vec::new();
        for i in 0..s.len() {
            if s[i] == g[i] {
                a += 1;
                s[i] = 99;
            } else {
                left.push(g[i]);
            }
        }
        for n in left {
            for i in 0..s.len() {
                if n == s[i] {
                    b += 1;
                    s[i] = 99;
                    break;
                }
            }
        }
        let mut ans: String = a.to_string();
        ans.push_str("A");
        ans.push_str(&b.to_string());
        ans.push_str("B");
        return ans;
    }
}
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

class Solution {
    public List<String> fizzBuzz(int n) {
        ArrayList<String> ans = new ArrayList<>();
        for(int i = 1; i <=n; i++) {
            String s = "";
            if(i % 3 == 0) {
                s += "Fizz";
            }
            if(i % 5 == 0) {
                s += "Buzz";
            }
            if(s.isEmpty()) {
                s = String.valueOf(i);
            }
            ans.add(s);
        }
        return ans;
    }
}
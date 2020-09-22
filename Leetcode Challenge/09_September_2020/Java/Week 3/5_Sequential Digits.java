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

class Solution {
    public List<Integer> sequentialDigits(int low, int high) {
        String s = "123456789";
        String sub = "";
        int ans = 0;
        List<Integer> v = new ArrayList<>();
        for(int i = 0; i < 9; i++) {
            for(int j = i; j < 9; j++) {
                sub = s.substring(i, j + 1);
                ans = Integer.parseInt(sub);
                if(ans >= low && ans <= high) {
                    v.add(ans);
                }
            }
        }
        Collections.sort(v);
        return v;
    }
}
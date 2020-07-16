//Given a non-empty array of digits representing a non-negative integer, increment one to the integer.
//
//The digits are stored such that the most significant digit is at the head of the list, and each element in the array contains a single digit.
//
//You may assume the integer does not contain any leading zero, except the number 0 itself.
//
//Example 1:
//
//Input: [1,2,3]
//Output: [1,2,4]
//Explanation: The array represents the integer 123.
//Example 2:
//
//Input: [4,3,2,1]
//Output: [4,3,2,2]
//Explanation: The array represents the integer 4321.

class Solution {
    public int[] plusOne(int[] digits) {
        List<Integer> l = new ArrayList<>();
        for(int e : digits) {
            l.add(e);
        }
        int i = l.size() - 1;
        l.set(i, l.get(i) + 1);
        while(i >= 0 && l.get(i) == 10) {
            l.set(i, 0);
            if(i == 0) {
                l.add(0, 1);
            } else {
                l.set(i - 1, l.get(i - 1) + 1);
            }
            i--;
        }
        int[] ans = l.stream().mapToInt(I->I).toArray();
        return ans;
    }
}
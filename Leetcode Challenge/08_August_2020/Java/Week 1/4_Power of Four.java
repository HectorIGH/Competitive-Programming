//Given an integer (signed 32 bits), write a function to check whether it is a power of 4.
//
//Example 1:
//
//Input: 16
//Output: true
//Example 2:
//
//Input: 5
//Output: false
//Follow up: Could you solve it without loops/recursion?

class Solution {
    public boolean isPowerOfFour(int num) {
        /*
        int i = 0;
        while(num != 0) {
            if((num & 1) == 1) {
                break;
            }
            i++;
            num >>= 1;
        }
        return num == 1 && (i & 1) == 0;
        */
        /*
        return num> 0 && num == (int)Math.pow(4, (int)(Math.log(num) / Math.log(4)));
        */
        return num > 0 && (num & (num - 1)) == 0 && (num & 0x55555555) != 0;
    }
}
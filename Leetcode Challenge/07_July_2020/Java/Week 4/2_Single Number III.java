//Given an array of numbers nums, in which exactly two elements appear only once and all the other elements appear exactly twice. Find the two elements that appear only once.
//
//Example:
//
//Input:  [1,2,1,3,2,5]
//Output: [3,5]
//Note:
//
//The order of the result is not important. So in the above example, [5, 3] is also correct.
//Your algorithm should run in linear runtime complexity. Could you implement it using only constant space complexity?


class Solution {
    public int[] singleNumber(int[] nums) {
        int s = Arrays.stream(nums).reduce(0, (e1, e2) -> e1 ^ e2);
        int leastbit = (s & (s - 1)) ^ s;
        int num1 = 0;
        for(int e : nums) {
            if((e & leastbit) != 0){
                num1 ^= e;
            }
        }
        int[] ans = {num1, s ^ num1};
        return ans;
    }
}
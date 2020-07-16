//Given a non-empty array of integers, every element appears three times except for one, which appears exactly once. Find that single one.
//
//Note:
//
//Your algorithm should have a linear runtime complexity. Could you implement it without using extra memory?
//
//Example 1:
//
//Input: [2,2,3,2]
//Output: 3
//Example 2:
//
//Input: [0,1,0,1,0,1,99]
//Output: 99

class Solution {
    public int singleNumber(int[] nums) {
        // Bitwise O(1) space
        /*
        int ones = 0, twos = 0, not_threes = 0;
        for(int n : nums) {
            twos |= ones & n;
            ones ^= n;
            not_threes = ~(ones & twos);
            ones &= not_threes;
            twos &= not_threes;
        }
        return ones;
        /*/
        // HashMap space O(n)
        HashMap<Integer, Integer> freq = new HashMap<>();
        for(int n : nums) {
            freq.put(n, freq.getOrDefault(n, 0) + 1);
        }
        for(Map.Entry<Integer, Integer> n : freq.entrySet()) {
        //for(int n : nums) {
            if(n.getValue() == 1) {
                return n.getKey();
            }
            //if(freq.get(n) == 1) {
                //return n;
            //}
        }
        return 0;
        
    }
}
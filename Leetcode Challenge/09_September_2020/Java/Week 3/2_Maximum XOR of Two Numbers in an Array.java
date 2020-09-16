//Given a non-empty array of numbers, a0, a1, a2, … , an-1, where 0 ≤ ai < 231.
//
//Find the maximum result of ai XOR aj, where 0 ≤ i, j < n.
//
//Could you do this in O(n) runtime?
//
//Example:
//
//Input: [3, 10, 5, 25, 2, 8]
//
//Output: 28
//
//Explanation: The maximum result is 5 ^ 25 = 28.

class Solution {
    public int findMaximumXOR(int[] nums) {
        Long ans = 0L, mask = 0L, start = 0L;
        
        HashSet<Long> found = new HashSet<>();
        for(long i = 31; i >= 0; i--) {
            mask |= 1L << Long.valueOf(i);
            found = new HashSet<>();
            for(int num : nums) {
                found.add(Long.valueOf(num) & Long.valueOf(mask));
            }
            start = ans | 1L << i;
            for(long pref : found) {
                if(found.contains(start ^ pref)) {
                    ans = start;
                    break;
                }
            }
        }
        return ans.intValue();
    }
}
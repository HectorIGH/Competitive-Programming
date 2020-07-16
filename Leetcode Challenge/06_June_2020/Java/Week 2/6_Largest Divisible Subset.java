//Given a set of distinct positive integers, find the largest subset such that every pair (Si, Sj) of elements in this subset satisfies:
//
//Si % Sj = 0 or Sj % Si = 0.
//
//If there are multiple solutions, return any subset is fine.
//
//Example 1:
//
//Input: [1,2,3]
//Output: [1,2] (of course, [1,3] will also be ok)
//Example 2:
//
//Input: [1,2,4,8]
//Output: [1,2,4,8]

class Solution {
    public List<Integer> largestDivisibleSubset(int[] nums) {
        int n = nums.length;
        if (n == 0) {
            return new ArrayList<Integer>();
        }
        Arrays.sort(nums);
        int[] divCount = new int[n];
        int[] prev = new int[n];
        List<Integer> ans = new ArrayList<>();
        int max_index = 0;
        Arrays.fill(divCount, 1);
        Arrays.fill(prev, -1);
        for(int i = 1; i < n; i++) {
            for(int j = 0; j < i; j++) {
                if(nums[i] % nums[j] == 0) {
                    if(divCount[i] <= divCount[j]) {
                        divCount[i] = divCount[j]+ 1;
                        prev[i] = j;
                    }
                }
            }
            if(divCount[max_index] < divCount[i]) {
                max_index = i;
            }
        }
        while (max_index >= 0) {
            ans.add(nums[max_index]);
            max_index = prev[max_index];
        }
        return ans;
    }
}
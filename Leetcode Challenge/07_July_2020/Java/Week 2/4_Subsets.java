//Given a set of distinct integers, nums, return all possible subsets (the power set).
//
//Note: The solution set must not contain duplicate subsets.
//
//Example:
//
//Input: nums = [1,2,3]
//Output:
//[
//  [3],
//  [1],
//  [2],
//  [1,2,3],
//  [1,3],
//  [2,3],
//  [1,2],
//  []
//]

class Solution {
    public List<List<Integer>> subsets(int[] nums) {
        /*
        List<List<Integer>> ans = new ArrayList();
        ans.add(new ArrayList());
        for(int num : nums) {
            List<List<Integer>> temp = new ArrayList();
            for(List<Integer> t : ans) {
                temp.add(new ArrayList<Integer>(t){{
                    add(num);
                }});
            }
            for(List<Integer> curr : temp) {
                ans.add(curr);
            }
        }
        return ans;
        */
        
        int n = nums.length;
        List<List<Integer>> ans = new ArrayList<>();
        String bm;
        for(int i = 1 << n; i < 1 << (n + 1); i++) {
            bm = Integer.toBinaryString(i).substring(1);
            List<Integer> t = new ArrayList<>();
            for(int j = 0; j < n; j++) {
                if(bm.charAt(j) == '1') {
                    t.add(nums[j]);
                }
            }
            ans.add(t);
        }
        return ans;
        
    }
}
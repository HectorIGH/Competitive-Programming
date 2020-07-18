//Given a non-empty array of integers, return the k most frequent elements.
//
//Example 1:
//
//Input: nums = [1,1,1,2,2,3], k = 2
//Output: [1,2]
//Example 2:
//
//Input: nums = [1], k = 1
//Output: [1]
//Note:
//
//You may assume k is always valid, 1 ≤ k ≤ number of unique elements.
//Your algorithm's time complexity must be better than O(n log n), where n is the array's size.
//It's guaranteed that the answer is unique, in other words the set of the top k frequent elements is unique.
//You can return the answer in any order.

class Solution {
    public int[] topKFrequent(int[] nums, int k) {
        if(k == nums.length) {
            return nums;
        }
        HashMap<Integer, Integer> freq = new HashMap<>();
        for(int n : nums) {
            freq.put(n, freq.getOrDefault(n, 0) + 1);
        }
        Queue<Integer> heap = new PriorityQueue<>((n1, n2) -> freq.get(n1) - freq.get(n2));
        for(int n : freq.keySet()) {
            heap.add(n);
            if(heap.size() > k) {
                heap.poll();
            }
        }
        int[] ans = new int[k];
        for(int i = 0; i < k; i++) {
            ans[i] = heap.poll();
        }
        return ans;
    }
}
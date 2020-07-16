//The set [1,2,3,...,n] contains a total of n! unique permutations.
//
//By listing and labeling all of the permutations in order, we get the following sequence for n = 3:
//
//"123"
//"132"
//"213"
//"231"
//"312"
//"321"
//Given n and k, return the kth permutation sequence.
//
//Note:
//
//Given n will be between 1 and 9 inclusive.
//Given k will be between 1 and n! inclusive.
//Example 1:
//
//Input: n = 3, k = 3
//Output: "213"
//Example 2:
//
//Input: n = 4, k = 9
//Output: "2314"

class Solution {
    public String getPermutation(int n, int k) {
        int[] factorial = new int[n];
        factorial[0] = 1;
        List<Integer> l = new ArrayList<>();
        l.add(1);
        StringBuilder ans = new StringBuilder();
        int j = 0;
        k--;
        
        for(int i = 1; i < n; i++) {
            factorial[i] = i * factorial[i - 1];
            l.add(i + 1);
        }
        
        for(int i = n - 1; i >= 0; i--) {
            j = k / factorial[i];
            k -= j * factorial[i];
            
            ans.append(l.get(j));
            l.remove(j);
        }
        return ans.toString();
    }
}
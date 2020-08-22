//Given an array A of non-negative integers, return an array consisting of all the even elements of A, followed by all the odd elements of A.
//
//You may return any answer array that satisfies this condition.
//
// 
//
//Example 1:
//
//Input: [3,1,2,4]
//Output: [2,4,3,1]
//The outputs [4,2,3,1], [2,4,1,3], and [4,2,1,3] would also be accepted.
// 
//
//Note:
//
//1 <= A.length <= 5000
//0 <= A[i] <= 5000

class Solution {
    public int[] sortArrayByParity(int[] A) {
        /*
        ArrayList<Integer> ans = new ArrayList<>();
        for(int n : A) {
            if((n & 1) != 0) {
                ans.add(n);
            } else {
                ans.add(0, n);
            }
        }
        int[] out = ans.stream().mapToInt(i -> i).toArray();
        return out;
        */
        int l = A.length - 1;
        for(int i = 0; i < l; i++) {
            if((A[i] & 1) != 0) {
                int aux = A[i];
                A[i] = A[l];
                A[l] = aux;
                i--;
                l--;
            }
        }
        return A;
    }
}
//Given an array of 4 digits, return the largest 24 hour time that can be made.
//
//The smallest 24 hour time is 00:00, and the largest is 23:59.  Starting from 00:00, a time is larger if more time has elapsed since midnight.
//
//Return the answer as a string of length 5.  If no valid time can be made, return an empty string.
//
// 
//
//Example 1:
//
//Input: [1,2,3,4]
//Output: "23:41"
//Example 2:
//
//Input: [5,5,5,5]
//Output: ""
// 
//
//Note:
//
//A.length == 4
//0 <= A[i] <= 9

class Solution {
    public String largestTimeFromDigits(int[] a) {
        ArrayList<Integer> A = new ArrayList<>(a.length);
        for(int i : a) {
            A.add(i);
        }
        Collections.sort(A); //A.sort()
        List<List<Integer>> perm = new ArrayList<>();
        List<Integer> indexes = new ArrayList<>();
        for(int i = 0; i < 4; i++) {
            indexes.add(i);
        }
        perms(indexes, 4, perm);
        int out = -1;
        String ans = "";
        int w, x, y, z;
        for(List<Integer> i : perm) {
            w = i.get(0);
            x = i.get(1);
            y = i.get(2);
            z = i.get(3);
            if(A.get(w) * 10 + A.get(x) <= 23 && A.get(y) <= 5) {
                out = Math.max(out, 60 * (A.get(w) * 10 + A.get(x)) + 10 * A.get(y) + A.get(z));
            }
        }
        if(out == -1) {
            return "";
        }
        int h = (int)(out / 60);
        int m = out % 60;
        if(h < 10) {
            ans += '0';
        }
        ans += Integer.toString(h) + ':';
        if(m < 10) {
            ans += '0';
        }
        ans += Integer.toString(m);
        return ans;
    }
    
    public void perms(List<Integer> a, int n, List<List<Integer>> p){
        if(n == 1) {
            List<Integer> dummy = new ArrayList(a);
            p.add(dummy);
            return ;
        }
        for(int i = 0; i < n; i++) {
            Collections.swap(a, i, n - 1);
            perms(a, n - 1, p);
            Collections.swap(a, i, n - 1);
        }
    }
}
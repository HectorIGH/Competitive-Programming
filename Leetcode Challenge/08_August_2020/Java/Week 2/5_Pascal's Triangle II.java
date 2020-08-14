//Given a non-negative index k where k ≤ 33, return the kth index row of the Pascal's triangle.
//
//Note that the row index starts from 0.
//
//
//In Pascal's triangle, each number is the sum of the two numbers directly above it.
//
//Example:
//
//Input: 3
//Output: [1,3,3,1]
//Follow up:
//
//Could you optimize your algorithm to use only O(k) extra space?

import java.math.BigInteger;

class Solution {
    public List<Integer> getRow(int rowIndex) {
        int n = rowIndex;
        Integer[] ans = new Integer[n + 1];
        double[] tes = new double[n / 2 + 1];
        tes[0] = 1;
        ans[0] = 1;
        for(int i = 1; i < n / 2 + 1; i++) {
            //ans[i] = ncr(n, i).intValue(); // i = 0..n/2 + 1
            tes[i] = tes[i - 1] * (n - i + 1) / i;
            ans[i] = (int)tes[i];
        }
        
        for(int i = (int)n / 2 + 1; i <= n; i++) {
            ans[i] = ans[n % i];
        }
            
        return Arrays.asList(ans);
    }
    /*
    public BigInteger ncr(int n, int r) {
        r = Math.min(r, n-r);
        BigInteger numer = BigInteger.ONE;
        for(int i = n; i > n - r; i--) {
            numer = numer.multiply(BigInteger.valueOf(i));
        }
        BigInteger denom = BigInteger.ONE;
        for(int i = 1; i < r + 1; i++) {
            denom = denom.multiply(BigInteger.valueOf(i));
        }
        return numer.divide(denom);
    }
    */
}
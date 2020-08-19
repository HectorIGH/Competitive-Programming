//Return all non-negative integers of length N such that the absolute difference between every two consecutive digits is K.
//
//Note that every number in the answer must not have leading zeros except for the number 0 itself. For example, 01 has one leading zero and is invalid, but 0 is valid.
//
//You may return the answer in any order.
//
// 
//
//Example 1:
//
//Input: N = 3, K = 7
//Output: [181,292,707,818,929]
//Explanation: Note that 070 is not a valid number, because it has leading zeroes.
//Example 2:
//
//Input: N = 2, K = 1
//Output: [10,12,21,23,32,34,43,45,54,56,65,67,76,78,87,89,98]
// 
//
//Note:
//
//1 <= N <= 9
//0 <= K <= 9

class Solution {
    public int[] numsSameConsecDiff(int N, int K) {
        if(N == 1) {
            int[] fast = new int[10];
            for(int i = 0; i < 10; i++) {
                fast[i] = i;
            }
            return fast;
        }
        int[] comb;
        if(K != 0) {
            comb  = new int[(int)Math.pow(2, N - 1)];
            for(int i = 0; i < Math.pow(2, N - 1); i++) {
                comb[i] = i;
            }
        } else {
            comb = new int[1];
            comb[0] = 0;
        }
        ArrayList<Integer> ans = new ArrayList<>();
        boolean good = true;
        for(int i = 1; i < 10; i++) {
            int[] n = new int[N];
            n[0] = i;
            for(int c : comb) {
                n = new int[N];
                n[0] = i;
                good = true;
                int m = i * (int)Math.pow(10, N - 1);
                for(int t = 1; t < N; t++) {
                    if((c & 1) == 0) {
                        if(n[t - 1] - K < 0) {
                            good = false;
                            break;
                        } else {
                            n[t] = n[t - 1] - K;
                            m += (n[t - 1] - K) * (int)Math.pow(10, N - 1 - t);
                        }
                    } else {
                        if(n[t - 1] + K > 9) {
                            good = false;
                            break;
                        } else {
                            n[t] = n[t - 1] + K;
                            m += (n[t - 1] + K) * (int)Math.pow(10, N - 1 - t);
                        }
                    }
                    c >>= 1;
                }
                if(good) {
                    ans.add(m);
                }
            }
        }
        int[] out = ans.stream().mapToInt(i -> i).toArray();
        return out;
    }
}
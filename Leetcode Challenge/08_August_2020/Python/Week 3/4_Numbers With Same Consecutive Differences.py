#Return all non-negative integers of length N such that the absolute difference between every two consecutive digits is K.
#
#Note that every number in the answer must not have leading zeros except for the number 0 itself. For example, 01 has one leading zero and is invalid, but 0 is valid.
#
#You may return the answer in any order.
#
# 
#
#Example 1:
#
#Input: N = 3, K = 7
#Output: [181,292,707,818,929]
#Explanation: Note that 070 is not a valid number, because it has leading zeroes.
#Example 2:
#
#Input: N = 2, K = 1
#Output: [10,12,21,23,32,34,43,45,54,56,65,67,76,78,87,89,98]
# 
#
#Note:
#
#1 <= N <= 9
#0 <= K <= 9

class Solution:
    def numsSameConsecDiff(self, N: int, K: int) -> List[int]:
        if N == 1:
            return list(range(10))
        if K:
            comb = [i for i in range(2 ** (N - 1))]
        else:
            comb = [0]
        #p = [bin(i)[2:] for i in comb]
        #print(comb)
        #print(p)
        ans = []
        good = True
        for i in range(1, 10): # starter digit
            n = [0] * N
            n[0] = i # candidate
            for c in comb: # combinations of operations
                good = True
                m = i * 10**(N - 1)
                for t in range(1, N):
                    # 0 is minus K, 1 is plus K
                    if c & 1 == 0: # 0 in current position
                        if n[t - 1] - K < 0:
                            good = False
                            break
                        else:
                            n[t] = n[t - 1] - K
                            m += (n[t - 1] - K) * 10**(N - 1 - t)
                    else: # 1 in current position
                        if n[t - 1] + K > 9:
                            good = False
                            break
                        else:
                            n[t] = n[t - 1] + K
                            m += (n[t - 1] + K) * 10**(N - 1 - t)
                    c >>= 1
                if good:
                    #print(n, "is good, and so do", m)
                    ans.append(m)
        return ans
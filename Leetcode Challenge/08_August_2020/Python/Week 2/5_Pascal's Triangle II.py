#Given a non-negative index k where k ≤ 33, return the kth index row of the Pascal's triangle.
#
#Note that the row index starts from 0.
#
#
#In Pascal's triangle, each number is the sum of the two numbers directly above it.
#
#Example:
#
#Input: 3
#Output: [1,3,3,1]
#Follow up:
#
#Could you optimize your algorithm to use only O(k) extra space?

class Solution:
    def getRow(self, rowIndex: int) -> List[int]:
        n = rowIndex
        ans = [1] * (n + 1)
        for i in range(1, n // 2 + 1):
            # n! / (i! * (n - i)!)
            #ans[i] = self.ncr(n, i) # math.comb(n, i) # i = 0..n // 2 + 1
            ans[i] = int(ans[i - 1] * (n - i + 1) / i)
        
        for i in range(n // 2 + 1, n + 1):
            ans[i] = ans[n % i]
            
        return ans
    
    def ncr(self, n, r):
        r = min(r, n-r)
        numer = reduce(mul, range(n, n-r, -1), 1)
        denom = reduce(mul, range(1, r+1), 1)
        return numer // denom
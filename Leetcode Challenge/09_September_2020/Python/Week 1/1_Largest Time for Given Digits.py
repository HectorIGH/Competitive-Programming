#Given an array of 4 digits, return the largest 24 hour time that can be made.
#
#The smallest 24 hour time is 00:00, and the largest is 23:59.  Starting from 00:00, a time is larger if more time has elapsed since midnight.
#
#Return the answer as a string of length 5.  If no valid time can be made, return an empty string.
#
# 
#
#Example 1:
#
#Input: [1,2,3,4]
#Output: "23:41"
#Example 2:
#
#Input: [5,5,5,5]
#Output: ""
# 
#
#Note:
#
#A.length == 4
#0 <= A[i] <= 9

class Solution:
    def largestTimeFromDigits(self, A: List[int]) -> str:
        A.sort()
        perms = []
        self.perms([0, 1, 2, 3], 4, perms)
        out = -1
        ans = ""
        for w, x, y, z in perms:
            if A[w] * 10 + A[x] <= 23 and A[y] <= 5:
                out = max(out, 60 * (A[w] * 10 + A[x]) + 10 * A[y] + A[z])
        if out == -1:
            return ""
        h = out // 60
        m = out % 60
        if h < 10:
            ans += '0'
        ans += str(h) + ':'
        if m < 10:
            ans += '0'
        ans += str(m)
        return ans
        
    def perms(self, a, n, p):
        if(n == 1):
            p.append(tuple(a))
            return 
        for i in range(n):
            a[i], a[n - 1] = a[n - 1], a[i]
            self.perms(a, n - 1, p)
            a[i], a[n - 1] = a[n - 1], a[i]
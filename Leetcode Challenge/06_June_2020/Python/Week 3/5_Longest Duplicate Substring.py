#Given a string S, consider all duplicated substrings: (contiguous) substrings of S that occur 2 or more times.  (The occurrences may overlap.)
#
#Return any duplicated substring that has the longest possible length.  (If S does not have a duplicated substring, the answer is "".)
#
# 
#
#Example 1:
#
#Input: "banana"
#Output: "ana"
#Example 2:
#
#Input: "abcd"
#Output: ""
# 
#
#Note:
#
#2 <= S.length <= 10^5
#S consists of lowercase English letters.
#   Hide Hint #1  
#Binary search for the length of the answer. (If there's an answer of length 10, then there are answers of length 9, 8, 7, ...)
#   Hide Hint #2  
#To check whether an answer of length K exists, we can use Rabin-Karp 's algorithm.

class Solution:
    def RabinKarp(self, text, M, q):
        if M == 0:
            return True
        h, t, d = (1<<(8*M-8))%q, 0, 256
        dic = defaultdict(list)
        for i in range(M):
            t = (d * t + ord(text[i])) % q
        dic[t].append(0)
        for i in range(len(text) - M):
            t = (d * (t - ord(text[i]) * h) + ord(text[i + M])) % q
            for j in dic[t]:
                if text[i+1:i+M+1] == text[j:j+M]:
                    return text[j:j+M]
            dic[t].append(i+1)
        return ""
    
    def longestDupSubstring(self, S: str) -> str:
        l, r = 0, len(S)
        q = (1<<31) - 1
        ans = ""
        while l + 1 < r:
            mid = l + (r - l) // 2
            candidate = self.RabinKarp(S, mid, q)
            if candidate:
                l, ans = mid, candidate
            else:
                r = mid
        return ans
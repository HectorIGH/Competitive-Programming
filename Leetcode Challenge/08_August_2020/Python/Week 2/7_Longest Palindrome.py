#Given a string which consists of lowercase or uppercase letters, find the length of the longest palindromes that can be built with those letters.
#
#This is case sensitive, for example "Aa" is not considered a palindrome here.
#
#Note:
#Assume the length of given string will not exceed 1,010.
#
#Example:
#
#Input:
#"abccccdd"
#
#Output:
#7
#
#Explanation:
#One longest palindrome that can be built is "dccaccd", whose length is 7.

class Solution:
    def longestPalindrome(self, s: str) -> int:
        #f = defaultdict(int)
        g = [0] * 58
        odd = False
        ans = 0
        for l in s:
            #f[l] += 1
            g[ord(l) - 65] += 1
            
        for l in g:
        #for l in f.values():
            if l & 1:
                if not odd:
                    ans += l
                    odd = True
                    continue
                else:
                    odd = True
            ans += (l >> 1) << 1
        return ans
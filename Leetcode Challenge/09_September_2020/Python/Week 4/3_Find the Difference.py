#Given two strings s and t which consist of only lowercase letters.
#
#String t is generated by random shuffling string s and then add one more letter at a random position.
#
#Find the letter that was added in t.
#
#Example:
#
#Input:
#s = "abcd"
#t = "abcde"
#
#Output:
#e
#
#Explanation:
#'e' is the letter that was added.

class Solution:
    def findTheDifference(self, s: str, t: str) -> str:
        '''
        sc = [0] * 26
        tc = [0] * 26
        for l in s:
            sc[ord(l) - 97] += 1
        for l in t:
            tc[ord(l) - 97] += 1
        for i in range(26):
            if sc[i] != tc[i]:
                return chr(i + 97)
        '''
        '''
        x = 0
        for l in s:
            x ^= ord(l)
        for l in t:
            x ^= ord(l)
        return chr(x)
        '''
        for l in t:
            if t.count(l) > s.count(l):
                return l
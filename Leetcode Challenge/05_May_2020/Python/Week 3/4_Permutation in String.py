#Given two strings s1 and s2, write a function to return true if s2 contains the permutation of s1. In other words, one of the first string's permutations is the substring of the second string.
#
# 
#
#Example 1:
#
#Input: s1 = "ab" s2 = "eidbaooo"
#Output: True
#Explanation: s2 contains one permutation of s1 ("ba").
#Example 2:
#
#Input:s1= "ab" s2 = "eidboaoo"
#Output: False
# 
#
#Constraints:
#
#The input strings only contain lower case letters.
#The length of both given strings is in range [1, 10,000].

class Solution:
    def checkInclusion(self, s1: str, s2: str) -> bool:
        #if not s1:
            #return True
        #if not s2:
            #return False
        if (window:=len(s1)) > (ls2:=len(s2)):
            return False
        al = 'abcdefghijklmnopqrstuvwxyz'
        p = {s : 0 for s in al}
        co = {s : 0 for s in al}
        for s in s1:
            p[s] += 1
        for i in range(window):
            co[s2[i]] += 1
        for i in range(ls2 - window + 1):
            #print(i, co)
            if co == p:
                return True
            else:
                if i + window > ls2 - 1:
                    return False
                    
                s = s2[i + window]
                co[s] += 1
                s = s2[i]
                co[s] = max(0, co[s] - 1)
                
        return False
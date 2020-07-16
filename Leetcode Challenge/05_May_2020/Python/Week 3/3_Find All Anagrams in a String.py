#Given a string s and a non-empty string p, find all the start indices of p's anagrams in s.
#
#Strings consists of lowercase English letters only and the length of both strings s and p will not be larger than 20,100.
#
#The order of output does not matter.
#
#Example 1:
#
#Input:
#s: "cbaebabacd" p: "abc"
#
#Output:
#[0, 6]
#
#Explanation:
#The substring with start index = 0 is "cba", which is an anagram of "abc".
#The substring with start index = 6 is "bac", which is an anagram of "abc".
#Example 2:
#
#Input:
#s: "abab" p: "ab"
#
#Output:
#[0, 1, 2]
#
#Explanation:
#The substring with start index = 0 is "ab", which is an anagram of "ab".
#The substring with start index = 1 is "ba", which is an anagram of "ab".
#The substring with start index = 2 is "ab", which is an anagram of "ab".

class Solution:
    def findAnagrams(self, s: str, p: str) -> List[int]:
        #def isAna(a, b):
            #return Counter(a) == Counter(b)
        #n = len(s)
        #m = len(p)
        #if n < m:
            #return []
        #if n == m:
            #if isAna(s, p):
                #return [0]
        #if len(Counter(s)) == 1:
            #if len(Counter(p)) == 1 and s[0] == p[0]:
                #return list(range(n))
            #return []
        #ans = []
        #i = 0
        #while i <= (n - m):
            #a = s[i:i + m]
            #if isAna(a, p):
                #ans.append(i)
                #while i + m < n:
                    #if s[i] == s[i + m]:
                        #i += 1
                        #ans.append(i)
                    #else:
                        #break
            #i += 1
        #return ans
        if len(s) < len(p):
            return []
        
        MAX = 256
        def compare(a, b):
            for i in range(MAX):
                if a[i] != b[i]:
                    return False
            return True
        m = len(p)
        n = len(s)
        countP = [0]*MAX
        countSW = [0]*MAX
        ans = []
        
        for i in range(m):
            countP[ord(p[i])] += 1
            countSW[ord(s[i])] += 1
        for i in range(m, n):
            if compare(countP, countSW):
                ans.append(i-m)
            countSW[ord(s[i])] += 1
            countSW[ord(s[i-m])] -= 1
        if compare(countP, countSW):
            ans.append(n-m)
        return ans
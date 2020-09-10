#Given a pattern and a string str, find if str follows the same pattern.
#
#Here follow means a full match, such that there is a bijection between a letter in pattern and a non-empty word in str.
#
#Example 1:
#
#Input: pattern = "abba", str = "dog cat cat dog"
#Output: true
#Example 2:
#
#Input:pattern = "abba", str = "dog cat cat fish"
#Output: false
#Example 3:
#
#Input: pattern = "aaaa", str = "dog cat cat dog"
#Output: false
#Example 4:
#
#Input: pattern = "abba", str = "dog dog dog dog"
#Output: false
#Notes:
#You may assume pattern contains only lowercase letters, and str contains lowercase letters that may be separated by a single space.

class Solution:
    def wordPattern(self, pattern: str, str: str) -> bool:
        string = str.split()
        if len(string) != len(pattern):
            return False
        
        w_map = {}
        c_map = {}
        for c, w in zip(pattern, string):
            if c in c_map:
                if c_map[c] != w:
                    return False
            else:
                if w in w_map:
                    return False
                w_map[w] = c
                c_map[c] = w
        '''
        n = len(string)
        m = len(pattern)
        s = defaultdict(list)
        p = defaultdict(list)
        for i in range(m):
            e = pattern[i]
            p[e].append(i)
        for i in range(n):
            e = string[i]
            s[e].append(i)
        for a, b in zip(s.values(), p.values()):
            if a != b:
                return False
        '''
        return True
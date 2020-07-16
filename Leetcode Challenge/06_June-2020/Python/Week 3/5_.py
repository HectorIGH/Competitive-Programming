#Given a string, find the first non-repeating character in it and return it's index. If it doesn't exist, return -1.
#
#Examples:
#
#s = "leetcode"
#return 0.
#
#s = "loveleetcode",
#return 2.
#Note: You may assume the string contain only lowercase letters.

class Solution:
    def firstUniqChar(self, s: str) -> int:
        #for i in range(len(s)):
            #if s.count(s[i]) == 1:
                #return i
        #return -1
        
        ####
        #visited = []
        #for l in s:
            #if l in visited:
                #continue
            #if s.count(l) == 1:
                #return s.index(l)
            #visited.append(l)
        #return -1
        
        ############
        #d = deque([])
        #counts = {}
        #for l in s:
            #counts[l] = counts.get(l, 0) + 1
            #if not l in d:
                #d.append(l)
        #for l in d:
            #if counts[l] == 1:
                #return s.index(l)
        #return -1
        
        #### Best way ####
        #from collections import Counter
        #counts = Counter(s)
        #for i, e in enumerate(s):
        #    if counts[e] == 1:
        #        return i
        #return -1
    
        #### Another faster
        #aset = set()
        #for i in range(len(s)):
        #    if s[i] in aset:
        #        continue
        #    else:
        #        aset.add(s[i])
        #    if s[i] in s[i + 1:]:
        #        continue
        #    else:
        #        return i
        #return -1
    
        #### Even another faster
        a = 'abcdefghijklmnopqrstuvwxyz'
        index = [s.index(l) for l in a if s.count(l) == 1]
        return min(index) if index else -1
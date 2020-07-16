#Given an arbitrary ransom note string and another string containing letters from all the magazines, write a function that will return true if the ransom note can be constructed from the magazines ; otherwise, it will return false.
#
#Each letter in the magazine string can only be used once in your ransom note.
#
#Note:
#You may assume that both strings contain only lowercase letters.
#
#canConstruct("a", "b") -> false
#canConstruct("aa", "ab") -> false
#canConstruct("aa", "aab") -> true

class Solution:
    def canConstruct(self, ransomNote: str, magazine: str) -> bool:
        
        #### Slow, but correct solution
        #if not len(ransomNote):
        #    return True
        #if len(ransomNote) > len(magazine):
        #    return False
        #ransomNote = deque(sorted(ransomNote))
        #magazine = deque(sorted(magazine))
        #while len(ransomNote):
        #    current = ransomNote[0]
        #    while len(magazine) and current != magazine[0]:
        #        magazine.popleft()
        #    while len(magazine) and len(ransomNote) and magazine[0] == ransomNote[0]:
        #        magazine.popleft()
        #        ransomNote.popleft()
        #    while len(magazine) and magazine[0] == current:
        #        magazine.popleft()
        #    if len(ransomNote) and ransomNote[0] == current:
        #        return False
        #return len(ransomNote) == 0 and len(magazine) >= 0
    
        #### Faster solution
        
        #if len(ransomNote) > len(magazine):
        #    return False
        #if not len(ransomNote):
        #    return True
        #ransom = {}
        #maga = {}
        #for e in ransomNote:
        #    ransom[e] = ransom.get(e, 0) + 1
        #for e in magazine:
        #    maga[e] = maga.get(e, 0) + 1
        #for e in ransom:
        #    if not e in maga or ransom[e] > maga[e]:
        #        return False
        #return True
        
        #### Even Faster
        
        #if len(ransomNote) > len(magazine):
        #    return False
        #if not len(ransomNote):
        #    return True
        
        #for e in set(ransomNote):
        #    if ransomNote.count(e) > magazine.count(e):
        #        return False
        #return True
    
        #### Even Faster
        
        counts = Counter(ransomNote)
        
        for l, c in counts.items():
            if c > magazine.count(l):
                return False
        return True
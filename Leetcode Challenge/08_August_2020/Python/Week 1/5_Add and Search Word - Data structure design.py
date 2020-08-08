#Design a data structure that supports the following two operations:
#
#void addWord(word)
#bool search(word)
#search(word) can search a literal word or a regular expression string containing only letters a-z or .. A . means it can represent any one letter.
#
#Example:
#
#addWord("bad")
#addWord("dad")
#addWord("mad")
#search("pad") -> false
#search("bad") -> true
#search(".ad") -> true
#search("b..") -> true
#Note:
#You may assume that all words are consist of lowercase letters a-z.
#
#   Hide Hint #1  
#You should be familiar with how a Trie works. If not, please work on this problem: Implement Trie (Prefix Tree) first.

class TreeNode:
    def __init__(self):
        self.children = {}
        self.endNode = False

class WordDictionary:

    def __init__(self):
        """
        Initialize your data structure here.
        """
        self.root = TreeNode()
        

    def addWord(self, word: str) -> None:
        """
        Adds a word into the data structure.
        """
        root = self.root
        for l in word:
            #root = root.children.setdefault(l, TreeNode())
            if l in root.children:
                root = root.children[l]
            else:
                root.children[l] = TreeNode()
                root = root.children[l]
        root.endNode = True
        

    def search(self, word: str) -> bool:
        """
        Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter.
        """
        def dfs(root, index):
            if index == len(word):
                return root.endNode
            if word[index] == '.':
                for child in root.children:
                    if dfs(root.children[child], index + 1):
                        return True
                    else: continue
            if word[index] in root.children:
                return dfs(root.children[word[index]], index + 1)
            return False
        return dfs(self.root, 0)
        


# Your WordDictionary object will be instantiated and called as such:
# obj = WordDictionary()
# obj.addWord(word)
# param_2 = obj.search(word)
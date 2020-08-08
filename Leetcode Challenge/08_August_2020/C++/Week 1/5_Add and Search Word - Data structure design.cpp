//Design a data structure that supports the following two operations:
//
//void addWord(word)
//bool search(word)
//search(word) can search a literal word or a regular expression string containing only letters a-z or .. A . means it can represent any one letter.
//
//Example:
//
//addWord("bad")
//addWord("dad")
//addWord("mad")
//search("pad") -> false
//search("bad") -> true
//search(".ad") -> true
//search("b..") -> true
//Note:
//You may assume that all words are consist of lowercase letters a-z.
//
//   Hide Hint #1  
//You should be familiar with how a Trie works. If not, please work on this problem: Implement Trie (Prefix Tree) first.

class TrieNode {
public:
    bool endNode;
    unordered_map<char, TrieNode*> children;
    TrieNode() {
        endNode = false;
    }
};

class WordDictionary {
public:
    /** Initialize your data structure here. */
    TrieNode* root;
    WordDictionary() {
        root = new TrieNode();
    }
    
    /** Adds a word into the data structure. */
    void addWord(string word) {
        TrieNode* current = root;
        for(char l : word) {
            if(current->children.count(l)) {
                current = current->children[l];
            } else {
                current->children[l] = new TrieNode();
                current = current->children[l];
            }
        }
        current->endNode = true;
    }
    
    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    bool search(string word) {
        return dfs(root, 0, word);
    }
    
    bool dfs(TrieNode* root, int i, string word) {
        if(i == word.size()) {
            return root->endNode;
        }
        if(word[i] == '.') {
            for(auto child : root->children) {
                if(dfs(child.second, i + 1, word)) {
                    return true;
                } else {
                    continue;
                }
            }
        }
        if(root->children.count(word[i])) {
            return dfs(root->children[word[i]], i + 1, word);
        }
        return false;
    }
};

/**
 * Your WordDictionary object will be instantiated and called as such:
 * WordDictionary* obj = new WordDictionary();
 * obj->addWord(word);
 * bool param_2 = obj->search(word);
 */
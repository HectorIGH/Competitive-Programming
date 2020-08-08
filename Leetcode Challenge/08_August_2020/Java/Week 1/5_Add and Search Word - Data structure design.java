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

class WordDictionary {
    
    class TrieNode {
        TrieNode[] children;
        boolean endNode;
        public TrieNode() {
            this.children = new TrieNode[26];
            this.endNode = false;
        }
    }

    /** Initialize your data structure here. */
    TrieNode root;
    public WordDictionary() {
        root = new TrieNode();
    }
    
    /** Adds a word into the data structure. */
    public void addWord(String word) {
        TrieNode current = root;
        for(int i = 0; i < word.length(); i++) {
            char c = word.charAt(i);
            if(current.children[c - 'a'] != null) {
                current = current.children[c - 'a'];
            } else {
                current.children[c - 'a'] = new TrieNode();
                current = current.children[c - 'a'];
            }
        }
        current.endNode = true;
    }
    
    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    public boolean search(String word) {
        return dfs(root, 0, word);
    }
    
    public boolean dfs(TrieNode root, int i, String word) {
        if(i >= word.length()) {
            return root.endNode;
        }
        char c = word.charAt(i);
        if(c == '.') {
            for(TrieNode child : root.children) {
                if(child != null && dfs(child, i + 1, word)) {
                    return true;
                } else {
                    continue;
                }
            }
            return false;
        }
        if(root.children[c - 'a'] != null) {
            return dfs(root.children[c - 'a'], i + 1, word);
        }
        return false;
        /*
        if(root == null || root.children[c - 'a'] == null) {
            return false;
        }
        return dfs(root.children[c - 'a'], i + 1, word);
        */
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * WordDictionary obj = new WordDictionary();
 * obj.addWord(word);
 * boolean param_2 = obj.search(word);
 */
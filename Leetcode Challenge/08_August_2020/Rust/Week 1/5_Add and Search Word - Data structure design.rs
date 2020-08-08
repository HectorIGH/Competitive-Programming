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

#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    endNode: bool,
}
#[derive(Default)]
struct WordDictionary {
    trie: TrieNode
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }
    
    /** Adds a word into the data structure. */
    fn add_word(&mut self, word: String) {
        let mut root = &mut self.trie;
        for &c in word.as_bytes() {
            root = root.children[(c - b'a') as usize].get_or_insert(Box::new(Default::default()));
        }
        root.endNode = true;
    }
    
    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    fn search(&self, word: String) -> bool {
        WordDictionary::dfs(&self.trie, word.as_bytes())
    }
    
    fn dfs(root: &TrieNode, word: &[u8]) -> bool {
        if let Some(&c) = word.first() {
            if c == b'.' {
                for child in &root.children {
                    if let Some(node) = child {
                        if WordDictionary::dfs(&node, &word[1..]) {
                            return true;
                        }
                    }
                }
            } else if let Some(node) = &root.children[(c - b'a') as usize] {
                return WordDictionary::dfs(&node, &word[1..]);
            }
            false
        } else {
            root.endNode
        }
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */
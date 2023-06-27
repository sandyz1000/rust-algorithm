#![allow(unused)]
use std::collections::HashMap;



/// ## 211. Design Add and Search Words Data Structure
/// https://leetcode.com/problems/design-add-and-search-words-data-structure/
///
/// Design a data structure that supports adding new words and finding if a string matches 
/// any previously added string.
///
/// Implement the WordDictionary class:
///
/// - WordDictionary() Initializes the object.
/// - void addWord(word) Adds word to the data structure, it can be matched later.
/// - bool search(word) Returns true if there is any string in the data structure that matches 
/// - word or false otherwise. word may contain dots '.' where dots can be matched with any letter.
///
/// Example:
/// ------
/// - Input
/// ["WordDictionary","addWord","addWord","addWord","search","search","search","search"]
/// [[],["bad"],["dad"],["mad"],["pad"],["bad"],[".ad"],["b.."]]
/// - Output
/// [null,null,null,null,false,true,true,true]
///
/// Explanation
/// WordDictionary wordDictionary = new WordDictionary();
/// wordDictionary.addWord("bad");
/// wordDictionary.addWord("dad");
/// wordDictionary.addWord("mad");
/// wordDictionary.search("pad"); // return False
/// wordDictionary.search("bad"); // return True
/// wordDictionary.search(".ad"); // return True
/// wordDictionary.search("b.."); // return True
///
///
/// Constraints:
/// --------------
/// - 1 <= word.length <= 25
/// - word in addWord consists of lowercase English letters.
/// - word in search consist of '.' or lowercase English letters.
/// - There will be at most 2 dots in word for search queries.
/// - At most 104 calls will be made to addWord and search.
struct WordDictionary {
    children: HashMap<char, WordDictionary>,
    end: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        let children: HashMap<char, WordDictionary> = HashMap::new();
        WordDictionary { children, end: false }
    }
    
    fn add_word(&mut self, word: String) {
        let mut root = self;
        for c in word.chars() {
            root = root.children.entry(c).or_insert(WordDictionary::new());
        }
        root.end = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut root = self;
        for (i, c) in word.chars().enumerate() {
            if root.children.contains_key(&c) {
                root = root.children.get(&c).unwrap();
            } else {
                // if char = "." run search for remaining portion of word on 
                // all of curr nodes children;
                if c == '.' {
                    for v in root.children.values() {
                        // Search rest of the word
                        let word_to_search = word
                            .chars()
                            .into_iter().skip(i + 1).collect::<String>();
                        if v.search(word_to_search) {
                            return true;
                        }
                    }
                }
                return false;    
            }
        }
        root.end
    }
}


/// ## 208. Implement Trie (Prefix Tree)
///
/// https://leetcode.com/problems/implement-trie-prefix-tree/description/
///
/// A trie (pronounced as "try") or prefix tree is a tree data structure used to efficiently 
/// store and retrieve keys in a dataset of strings. There are various applications of this 
/// data structure, such as autocomplete and spellchecker.
///
/// Implement the Trie class:
///
/// - Trie() Initializes the trie object.
/// - void insert(String word) Inserts the string word into the trie.
/// - boolean search(String word) Returns true if the string word is in the trie (i.e., was 
/// - inserted before), and false otherwise.
/// - boolean startsWith(String prefix) Returns true if there is a previously inserted string 
/// - word that has the prefix prefix, and false otherwise.
///  
///
/// Example 1:
/// -----------
/// - Input
/// ["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
/// `[[]], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]`
/// - Output
/// [null, null, true, false, true, null, true]
///
/// #### Explanation
/// Trie trie = new Trie();
/// trie.insert("apple");
/// trie.search("apple");   // return True
/// trie.search("app");     // return False
/// trie.startsWith("app"); // return True
/// trie.insert("app");
/// trie.search("app");     // return True
#[derive(Debug, Clone, Default)]
struct Trie {
    children: HashMap<char, Trie>,
    end: bool,
}


impl Trie {
    fn new() -> Self {
        Self::default()        
    }
    
    fn insert(&mut self, word: String) {
        let mut link = self;
        for c in word.chars() {
            link = link.children.entry(c).or_insert(Trie::new());
        }
        link.end = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut link = self;
        for c in word.chars() {
            match link.children.get(&c) {
                Some(next) => {
                    link = next;
                },
                None => return false
            }
        }
        link.end
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut link = self;
        for c in prefix.chars() {
            match link.children.get(&c) {
                Some(next) => {
                    link = next;
                }
                None => return false,
            }
        }
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion() {
        let mut trie = Trie::new();
        trie.insert("apple".to_owned());
        assert_eq!(trie.search("apple".to_owned()), true);   // return True
        assert_eq!(trie.search("app".to_owned()), false);     // return False
        assert_eq!(trie.starts_with("app".to_owned()), true); // return True
        trie.insert("app".to_owned());
        assert_eq!(trie.search("app".to_owned()), true);     // return True

    }

    #[test]
    fn test_get() {
        let mut trie = Trie::new();
        trie.insert("foo".to_owned());
        trie.insert("foobar".to_owned());
        trie.insert("bar".to_owned());
        trie.insert("baz".to_owned());

        assert_eq!(trie.search("foo".to_owned()), true);
        assert_eq!(trie.search("food".to_owned()), false);

    }

    #[test]
    fn test_insert_trie() {
        //Your WordDictionary object will be instantiated and called as such:
        let mut word_dictionary = WordDictionary::new();
        word_dictionary.add_word("bad".to_string());
        word_dictionary.add_word("dad".to_string());
        word_dictionary.add_word("mad".to_string());
        assert_eq!(word_dictionary.search("pad".to_string()), false);
        assert_eq!(word_dictionary.search("bad".to_string()), true); 
        assert_eq!(word_dictionary.search(".ad".to_string()), true);
        assert_eq!(word_dictionary.search("b..".to_string()), true);
    }
}



/* 
https://leetcode.com/problems/implement-trie-prefix-tree/description/

A trie (pronounced as "try") or prefix tree is a tree data structure used to efficiently 
store and retrieve keys in a dataset of strings. There are various applications of this 
data structure, such as autocomplete and spellchecker.

Implement the Trie class:

Trie() Initializes the trie object.
void insert(String word) Inserts the string word into the trie.
boolean search(String word) Returns true if the string word is in the trie (i.e., was 
inserted before), and false otherwise.
boolean startsWith(String prefix) Returns true if there is a previously inserted string 
word that has the prefix prefix, and false otherwise.
 

Example 1:

Input
["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
[[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]
Output
[null, null, true, false, true, null, true]

Explanation
Trie trie = new Trie();
trie.insert("apple");
trie.search("apple");   // return True
trie.search("app");     // return False
trie.startsWith("app"); // return True
trie.insert("app");
trie.search("app");     // return True

 */
#![allow(dead_code)]
#![allow(unused_variables)]
use std::{collections::HashMap};


#[derive(Debug, Clone, Default)]
struct Trie {
    children: HashMap<char, Trie>,
    end: bool,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Self::default()        
    }
    
    fn insert(&mut self, word: String) {
        let mut link = self;
        for c in word.chars() {
            link = link.children.entry(c).or_default();
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
}

#![allow(unused)]

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
struct WordDictionary {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        unimplemented!()
    }

    
    fn add_word(&self, word: String) {
        unimplemented!()
    }

    fn search(&self, word: String) -> bool {
        unimplemented!()
    }
}

/*
* Your WordDictionary object will be instantiated and called as such:
* let obj = WordDictionary::new();
* obj.add_word(word);
* let ret_2: bool = obj.search(word);
*/
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {}
}

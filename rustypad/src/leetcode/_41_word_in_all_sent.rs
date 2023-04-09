/* 
Count of words that are present in all the given sentences

Given n sentences. The task is to count the number of words that appear in all of 
these sentences. 
Note that every word consists of only lowercase English alphabets.

 */

use std::collections::HashMap;

#[derive(Debug)]
struct Solution;

impl Solution {

    /// - create a dictionary of word as key and documents index as value
    /// - iterate each word and check if documents >= k then add to result
    fn get_top_words(documents: Vec<&str>, k: i32) -> Vec<&str> {
        let mut results: Vec<&str> = Vec::new();
        let mut bow: HashMap<&str, Vec<usize>> = HashMap::new();
        for (doc_id, doc) in documents.iter().enumerate() {
            let tokens: Vec<&str> = doc.split(' ').collect();
            for token in tokens {
                match bow.get_mut(token) {
                    Some(bow_docs) => bow_docs.push(doc_id),
                    None => { bow.insert(token, vec![doc_id]); },
                }
            }
        }

        for (word, document_ids) in bow {
            if document_ids.len() >= k as usize {
                results.push(word)
            }
        }

        return results;
    }
}

#[test]
fn test() {
    let arr = vec!["there is a cow",
           "cow is our mother",
           "cow gives us milk and milk is sweet",
           "there is a boy who loves cow"];
    let k = arr.len();
    let ans = Solution::get_top_words(arr, k as i32);
}
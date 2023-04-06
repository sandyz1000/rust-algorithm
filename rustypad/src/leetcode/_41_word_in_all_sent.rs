/* 

Count of words that are present in all the given sentences

Given n sentences. The task is to count the number of words that appear in all of 
these sentences. 
Note that every word consists of only lowercase English alphabets.

 */

#[derive(Debug)]
struct Solution;

impl Solution {

    /// - create a dictionary of word as key and documents index as value
    /// - iterate each word and check if documents >= k then add to result
    fn get_top_words(documents: Vec<&str>, k: i32) -> Vec<&str> {
        /* 
        bow = defaultdict(list)
        for doc_id, doc in enumerate(documents):
            for token in doc.split():
                bow[token].append(doc_id)
        results = [word for word, doc_ids in bow.items() if len(doc_ids) >= k]
        return results

         */
        todo!()
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
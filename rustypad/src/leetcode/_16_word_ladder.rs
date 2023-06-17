/* 


 */
#![allow(unused)]
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug)]
struct Solution;

type Graph = HashMap<String, Vec<String>>;

/// ### Build final graph from word bucket. 
/// Connect every word to it's associative word that differ from each other by
/// one letter. 
/// This will be useful for graph traversal from source word to dest word
fn build_final_word_graph(wordg: Graph) -> Graph {
    let mut final_word_graph: Graph = HashMap::new();
    for (k, values) in wordg.iter() {
        let size = values.len();
        for i in 0..size {
            for j in i+1..size {
                let x = final_word_graph.
                    entry(values[i].clone())
                    .or_insert(vec![values[j].clone()]);

                x.push(values[j].clone());

                let y = final_word_graph.
                    entry(values[j].clone())
                    .or_insert(vec![values[i].clone()]);

                y.push(values[i].clone());
                
            }
        }
    }
    final_word_graph
}

impl Solution {
    
    /// 1. Build a graph with node with same intermediate key form a connection
    /// 2. bfs from start node to dest node
    #[allow(dead_code)]
    fn word_ladder<'q>(
        begin_word: &'q str, end_word: &'q str, mut word_list: Vec<&'q str>, word_length: usize
    ) -> i32 {
        
        let mut word_graph: Graph = HashMap::new();
        word_list.push(begin_word);
        
        for word in word_list {
            for i in 0..word_length {
                let (x, y) = (&word[..(i as usize)], &word[((i as usize)+1)..]);

                let key = format!("{}${}", x, y);
                
                let node = {
                    let entry = word_graph.entry(key);
                    entry.or_insert(vec![word.to_string()])
                };
                node.push(word.to_string());
            }
        }

        let final_wg = build_final_word_graph(word_graph);
        return Solution::bfs(final_wg, begin_word, end_word);
    }

    fn bfs<'a>(wg: Graph, begin_word: &'a str, end_word: &'a str) -> i32 {
        let mut __q: VecDeque<(&str, i32)> = VecDeque::new();
        __q.push_back((begin_word, 1));
        let mut visited: HashSet<&str> = HashSet::new();
        visited.insert(begin_word);
        while !__q.is_empty() {
            let (curr_word, curr_depth) = __q.pop_front().unwrap();
            if curr_word == end_word {
                return curr_depth;
            }
            
            for conn in wg.get(curr_word).unwrap() {
                if visited.contains(conn.as_str()) {
                    __q.push_back((conn, curr_depth + 1));
                    visited.insert(conn);
                }
            }
        }

        0
    }
}

#[test]
fn test() {
    // TODO: Fix this test cases
    let begin_word = "hit";
    let end_word = "cog";
    let word_list = vec!["hot", "dot", "dog", "lot", "log", "cog"];
    let word_length = word_list[0].len();
    let ans = Solution::word_ladder(begin_word, end_word, word_list, word_length);
    
}
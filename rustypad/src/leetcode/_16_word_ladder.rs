/* 


def solution_bfs(beginWord: str, endWord: str, wordList: List[str]) -> int:
    
    word_length = len(wordList[0])
    word_graph = defaultdict(list)
    wordList.append(beginWord)
    for word in wordList:
        for i in range(word_length):
            intermediate_key = word[:i] + '$' + word[i + 1:]
            word_graph[intermediate_key].append(word)

    final_word_graph = defaultdict(list)
    for k, values in word_graph.items():
        for i in range(len(values)):
            for j in range(i + 1, len(values)):
                final_word_graph[values[i]].append(values[j])
                final_word_graph[values[j]].append(values[i])

    del word_graph

    def bfs(wg: Dict[str, List]):
        q = deque()
        q.append((beginWord, 1))
        visited = set()
        visited.add(beginWord)
        while q:
            curr_word, curr_depth = q.popleft()
            if curr_word == endWord:
                return curr_depth
            for conn in wg[curr_word]:
                if conn not in visited:
                    q.append((conn, curr_depth + 1))
                    visited.add(conn)
        return 0

    return bfs(final_word_graph)


 */

use std::collections::HashMap;

#[derive(Debug)]
struct Solution;


impl Solution {
    
    /// 1. Build a graph with node with same intermediate key form a connection
    /// 2. bfs from start node to dest node
    fn word_ladder(begin_word: &str, end_word: &str, word_list: Vec<&str>) -> i32 {
        todo!()
    }

    fn bfs(wg: HashMap<&str, Vec<&str>>) -> i32 {
        todo!()
    }
}

#[test]
fn test() {
    let begin_word = "hit";
    let end_word = "cog";
    let word_list = vec!["hot", "dot", "dog", "lot", "log", "cog"];

    let ans = Solution::word_ladder(begin_word, end_word, word_list);

}
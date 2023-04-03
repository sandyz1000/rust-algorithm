from collections import defaultdict
from typing import Dict, List
from queue import deque


def solution_bfs(beginWord: str, endWord: str, wordList: List[str]) -> int:
    # 1. Build a graph with node with same intermediate key form a connection
    # 2. bfs from start node to dest node
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


def main():
    beginWord = "hit"
    endWord = "cog"
    wordList = ["hot", "dot", "dog", "lot", "log", "cog"]

    print(solution_bfs(beginWord, endWord, wordList))


if __name__ == "__main__":
    main()

"""
https://leetcode.com/problems/evaluate-division/description/

399. Evaluate Division

You are given an array of variable pairs equations and an array of real numbers 
values, where equations[i] = [Ai, Bi] and values[i] represent the equation 
Ai / Bi = values[i]. Each Ai or Bi is a string that represents a single variable.

You are also given some queries, where queries[j] = [Cj, Dj] represents the jth 
query where you must find the answer for Cj / Dj = ?.

Return the answers to all queries. If a single answer cannot be determined, 
return -1.0.

Note: The input is always valid. You may assume that evaluating the queries will 
not result in division by zero and that there is no contradiction.


Example 1:
---------
Input:
equations = [["a","b"],["b","c"]]
values = [2.0,3.0]
queries = [["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]]
Output: [6.00000,0.50000,-1.00000,1.00000,-1.00000]
Explanation:
Given: a / b = 2.0, b / c = 3.0
queries are: a / c = ?, b / a = ?, a / e = ?, a / a = ?, x / x = ?
return: [6.0, 0.5, -1.0, 1.0, -1.0 ]

Example 2:
---------
Input: equations = [["a","b"],["b","c"],["bc","cd"]]
values = [1.5,2.5,5.0]
queries = [["a","c"],["c","b"],["bc","cd"],["cd","bc"]]
Output: [3.75000,0.40000,5.00000,0.20000]

Example 3:
---------
Input: equations = [["a","b"]]
values = [0.5]
queries = [["a","b"],["b","a"],["a","c"],["x","y"]]
Output: [0.50000,2.00000,-1.00000,-1.00000]

"""
from typing import List, DefaultDict
from collections import defaultdict, deque


def calcEquation(
    equations: List[List[str]],
    values: List[float],
    queries: List[List[str]]
) -> List[float]:
    # build graph of equations and values
    # iterate the queries for each src and dest and return the ans

    graph = defaultdict(dict)
    for node, weight in zip(equations, values):
        graph[node[0]][node[1]] = weight
        graph[node[1]][node[0]] = 1.0 / weight

    ans = []
    for dividend, divisor in queries:
        if dividend not in graph or divisor not in graph:
            ans.append(-1.0)
        elif dividend == divisor:
            ans.append(1.0)
        else:
            ans.append(bfs(graph, dividend, divisor))
        
    return ans


def bfs(graph: DefaultDict, source: int, target: int):
    __q = deque([(source, 1)])
    visited = set([source])
    while __q:
        node, prod = __q.popleft()
        # If target reached
        if node == target:
            return prod
        
        # For each divisor and quotient
        for ch, value in graph[node].items():
            if ch not in visited:
                visited.add(ch)
                __q.append((ch, value * prod))

    return -1


def cli_main():
    print(">>> Executing Test-1 >>>")
    # a / c = (a / b) * (b / c)
    #
    equations = [["a", "b"], ["b", "c"]]
    values = [2.0, 3.0]
    queries = [["a", "c"], ["b", "a"], ["a", "e"], ["a", "a"], ["x", "x"]]
    output = [6.00000, 0.50000, -1.00000, 1.00000, -1.00000]
    ans = calcEquation(equations, values, queries)
    assert ans == output

    print(">>> Executing Test-2 >>>")
    equations = [["a", "b"], ["b", "c"], ["bc", "cd"]]
    values = [1.5, 2.5, 5.0]
    queries = [["a", "c"], ["c", "b"], ["bc", "cd"], ["cd", "bc"]]
    output = [3.75000, 0.40000, 5.00000, 0.20000]
    ans = calcEquation(equations, values, queries)
    assert ans == output

    print(">>> Executing Test-3 >>>")
    equations = [["a", "b"]]
    values = [0.5]
    queries = [["a", "b"], ["b", "a"], ["a", "c"], ["x", "y"]]
    output = [0.50000, 2.00000, -1.00000, -1.00000]
    ans = calcEquation(equations, values, queries)
    assert ans == output


if __name__ == "__main__":
    cli_main()
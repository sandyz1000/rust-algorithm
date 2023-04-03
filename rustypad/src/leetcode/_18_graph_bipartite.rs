/*
https://leetcode.com/problems/is-graph-bipartite/description/

785. Is Graph Bipartite?

There is an undirected graph with n nodes, where each node is numbered between 
0 and n - 1. You are given a 2D array graph, where graph[u] is an array of nodes 
that node u is adjacent to. More formally, for each v in graph[u], there is an 
undirected edge between node u and node v. The graph has the following properties:

- There are no self-edges (graph[u] does not contain u).
- There are no parallel edges (graph[u] does not contain duplicate values).
- If v is in graph[u], then u is in graph[v] (the graph is undirected).
- The graph may not be connected, meaning there may be two nodes u and v such that 
there is no path between them.

A graph is bipartite if the nodes can be partitioned into two independent sets A 
and B such that every edge in the graph connects a node in set A and a node in set B.

Return true if and only if it is bipartite.

Example-1:
----------
Input: graph = [[1,2,3],[0,2],[0,1,3],[0,2]]
Output: false
Explanation: There is no way to partition the nodes into two independent sets such 
that every edge connects a node in one and a node in the other.

Example-2:
----------
Input: graph = [[1,3],[0,2],[1,3],[0,2]]
Output: true
Explanation: We can partition the nodes into two sets: {0, 2} and {1, 3}.

Constraints:
------------
graph.length == n
1 <= n <= 100
0 <= graph[u].length < n
0 <= graph[u][i] <= n - 1
graph[u] does not contain u.
All the values of graph[u] are unique.
If graph[u] contains v, then graph[v] contains u.

**/

use std::vec::Vec;
use std::collections::HashSet;

#[derive(Debug)]
struct Solution;

struct Graph {
    // Adjacency list impl of graph
    nodes: Vec<i32>,
    edges: Vec<HashSet<usize>>,
    n: usize
}

impl Graph {
    fn new(n: usize) -> Self {
        let edges: Vec<HashSet<usize>> = vec![HashSet::new(); n];
        let nodes: Vec<i32> = vec![0; n];
        return Graph { nodes, edges, n };
    }

    fn insert_edges(&mut self, u: usize, v: usize) -> bool {
        todo!();
    }

    fn dfs(&mut self, u: usize, color: i32) -> bool {
        todo!();
    }
}

impl Solution {
    fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        todo!();
    }
}



#[test]
fn test() {
    let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
    let res = true;
    assert_eq!(Solution::is_bipartite(graph), res);
    let graph = vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]];
    let res = false;
    assert_eq!(Solution::is_bipartite(graph), res);
    let graph = vec![
        vec![],
        vec![2, 4, 6],
        vec![1, 4, 8, 9],
        vec![7, 8],
        vec![1, 2, 8, 9],
        vec![6, 9],
        vec![1, 5, 7, 8, 9],
        vec![3, 6, 9],
        vec![2, 3, 4, 6, 9],
        vec![2, 4, 5, 6, 7, 8],
    ];
    let res = false;
    assert_eq!(Solution::is_bipartite(graph), res);
}

#![allow(unused)]
use std::vec::Vec;
use std::collections::{HashMap, VecDeque, HashSet};

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
    /// ## 785. Is Graph Bipartite?
    /// https://leetcode.com/problems/is-graph-bipartite/description/
    ///
    /// There is an undirected graph with n nodes, where each node is numbered between 
    /// 0 and n - 1. You are given a 2D array graph, where graph[u] is an array of nodes 
    /// that node u is adjacent to. More formally, for each v in graph[u], there is an 
    /// undirected edge between node u and node v. The graph has the following properties:
    ///
    /// - There are no self-edges (graph[u] does not contain u).
    /// - There are no parallel edges (graph[u] does not contain duplicate values).
    /// - If v is in graph[u], then u is in graph[v] (the graph is undirected).
    /// - The graph may not be connected, meaning there may be two nodes u and v such that 
    /// there is no path between them.
    ///
    /// A graph is bipartite if the nodes can be partitioned into two independent sets A 
    /// and B such that every edge in the graph connects a node in set A and a node in set B.
    ///
    /// Return true if and only if it is bipartite.
    ///
    /// Example-1:
    /// ----------
    /// ```
    /// let graph = vec![vec![1,2,3], vec![0,2], vec![0,1,3], vec![0,2]];
    /// assert_eq!(Solution::is_bipartite(graph), false);
    /// ```
    /// *Explanation*: There is no way to partition the nodes into two independent sets such 
    /// that every edge connects a node in one and a node in the other.
    ///
    /// Example-2:
    /// ----------
    /// ```
    /// let graph = vec![vec![1,3], vec![0,2], vec![1,3], vec![0,2]];
    /// assert_eq!(Solution::is_bipartite(graph), true);
    /// ```
    /// *Explanation*: We can partition the nodes into two sets: {0, 2} and {1, 3}.
    ///
    /// Constraints:
    /// ------------
    /// * graph.length == n
    /// * 1 <= n <= 100
    /// * 0 <= graph[u].length < n
    /// * 0 <= graph[u][i] <= n - 1
    /// * graph[u] does not contain u.
    /// * All the values of graph[u] are unique.
    /// * If graph[u] contains v, then graph[v] contains u.
    ///
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        // This will store the color of each node while visiting the graph
        let visited: HashMap<i32, bool> = HashMap::new();

        for i in 0..graph.len() {
            if !visited.contains_key(&(i as i32)) {
                if !Self::check_bfs(&graph, i as i32, &visited) {
                    return false;
                }
            }
        }
        true
    }

    // This will check if the node coloring is valid
    fn check_bfs(graph: &Vec<Vec<i32>>, node: i32, visited: &HashMap<i32, bool>) -> bool {
        let mut deq: VecDeque<(i32, bool)> = VecDeque::new();
        deq.push_back((node, true));
        while !deq.is_empty() {
            let (cur, color) = deq.pop_front().unwrap();

            // Check if the node already visited and the color is same
            if visited.contains_key(&cur) {
                if color != *visited.get(&cur).unwrap() {
                    return false;
                }
                continue;
            }

            for child in graph[cur as usize].iter() {
                if !visited.contains_key(child) {
                    // Assign the opp color to neighboring nodes
                    deq.push_back((*child, !color));
                }
            }
        }
        true
    }

    fn bfs(graph: &Vec<Vec<i32>>) -> bool {
        let mut color = vec![-1; graph.len()];
    
        for node in 0..graph.len() {
            if color[node] != -1 {
                continue;
            }
    
            let mut queue = VecDeque::new();
            queue.push_back(node);
            color[node] = 0;
    
            while let Some(current) = queue.pop_front() {
                for &c in &graph[current] {
                    if color[c as usize] == -1 {
                        queue.push_back(c as usize);
                        color[c as usize] = color[current] ^ 1;
                    } else if color[c as usize] == color[current] {
                        return false;
                    }
                }
            }
        }
    
        true
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

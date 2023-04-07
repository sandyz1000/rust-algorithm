/* 

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

 */

use std::collections::{HashMap, BinaryHeap, HashSet};


struct Solution;

type GraphType = HashMap<String, HashMap<i32, f32>>;

impl Solution {
    
    /// build graph of equations and values
    /// iterate the queries for each src and dest and return the ans
    fn calc_equation(
        equations: Vec<[&str; 2]>,
        values: Vec<f32>,
        queries: Vec<[&str; 2]>
    ) -> Vec<f32> {
        // graph = defaultdict(dict)
        // for node, weight in zip(equations, values):
        //     graph[node[0]][node[1]] = weight
        //     graph[node[1]][node[0]] = 1.0 / weight
    
        let mut ans: Vec<f32> = Vec::new();
        // for dividend, divisor in queries:
        //     if dividend not in graph or divisor not in graph:
        //         ans.append(-1.0)
        //     elif dividend == divisor:
        //         ans.append(1.0)
        //     else:
        //         ans.append(bfs(graph, dividend, divisor))
            
        return ans

    }
    
    fn bfs(graph: GraphType, source: i32, target: i32) -> i32 {
        let mut __q: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        let mut visited: HashSet<i32> = HashSet::new();
        __q.push((source, 1));
        visited.insert(source);
        
        // while __q:
        //     node, prod = __q.popleft()
        //     // If target reached
        //     if node == target:
        //         return prod
            
        //     // For each divisor and quotient
        //     for ch, value in graph[node].items():
        //         if ch not in visited:
        //             visited.add(ch)
        //             __q.append((ch, value * prod))
    
        return -1

    }
    
}

#[test]
fn test1() {
    println!(">>> Executing Test-1 >>>");
    // a / c = (a / b) * (b / c)
    let equations = vec![["a", "b"], ["b", "c"]];
    let values = vec![2.0, 3.0];
    let queries = vec![["a", "c"], ["b", "a"], ["a", "e"], ["a", "a"], ["x", "x"]];
    let output = vec![6.00000, 0.50000, -1.00000, 1.00000, -1.00000];
    let ans = Solution::calc_equation(equations, values, queries);
    assert_eq!(ans, output);

}

#[test]
fn test2() {
    println!(">>> Executing Test-2 >>>");
    let equations = vec![["a", "b"], ["b", "c"], ["bc", "cd"]];
    let values = vec![1.5, 2.5, 5.0];
    let queries = vec![["a", "c"], ["c", "b"], ["bc", "cd"], ["cd", "bc"]];
    let output = vec![3.75000, 0.40000, 5.00000, 0.20000];
    let ans = Solution::calc_equation(equations, values, queries);
    assert_eq!(ans, output);
}

#[test]
fn test3() {
    println!(">>> Executing Test-3 >>>")
    let equations = vec![["a", "b"]];
    let values = vec![0.5];
    let queries = vec![["a", "b"], ["b", "a"], ["a", "c"], ["x", "y"]];
    let output = vec![0.50000, 2.00000, -1.00000, -1.00000];
    let ans = Solution::calc_equation(equations, values, queries);
    assert_eq!(ans, output);
}


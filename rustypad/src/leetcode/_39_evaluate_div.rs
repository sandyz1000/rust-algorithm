use std::collections::{HashMap, VecDeque, HashSet};


struct Solution;

type Graph = HashMap<String, Vec<(String, f32)>>;

impl Solution {
    
    /// ## 399. Evaluate Division
    /// https://leetcode.com/problems/evaluate-division/description/
    ///
    ///
    /// You are given an array of variable pairs equations and an array of real numbers 
    /// values, where equations[i] = [Ai, Bi] and values[i] represent the equation 
    /// Ai / Bi = values[i]. Each Ai or Bi is a string that represents a single variable.
    ///
    /// You are also given some queries, where queries[j] = [Cj, Dj] represents the jth 
    /// query where you must find the answer for Cj / Dj = ?.
    ///
    /// Return the answers to all queries. If a single answer cannot be determined, 
    /// return -1.0.
    ///
    /// Note: The input is always valid. You may assume that evaluating the queries will 
    /// not result in division by zero and that there is no contradiction.
    ///
    ///
    /// Example 1:
    /// ---------
    /// Input:
    /// equations = [["a","b"],["b","c"]]
    /// values = [2.0,3.0]
    /// queries = [["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]]
    /// Output: [6.00000,0.50000,-1.00000,1.00000,-1.00000]
    /// Explanation:
    /// Given: a / b = 2.0, b / c = 3.0
    /// queries are: a / c = ?, b / a = ?, a / e = ?, a / a = ?, x / x = ?
    /// return: [6.0, 0.5, -1.0, 1.0, -1.0 ]
    ///
    /// Example 2:
    /// ---------
    /// Input: equations = [["a","b"],["b","c"],["bc","cd"]]
    /// values = [1.5,2.5,5.0]
    /// queries = [["a","c"],["c","b"],["bc","cd"],["cd","bc"]]
    /// Output: [3.75000,0.40000,5.00000,0.20000]
    ///
    /// Example 3:
    /// ---------
    /// Input: equations = [["a","b"]]
    /// values = [0.5]
    /// queries = [["a","b"],["b","a"],["a","c"],["x","y"]]
    /// Output: [0.50000,2.00000,-1.00000,-1.00000]
    /// 
    /// Algorithm:
    /// build graph of equations and values
    /// iterate the queries for each src and dest and return the ans
    #[allow(dead_code)]
    fn calc_equation(
        equations: Vec<[&str; 2]>,
        values: Vec<f32>,
        queries: Vec<[&str; 2]>
    ) -> Vec<f32> {
        let mut graph: Graph = HashMap::new();
        for (node, weight) in equations.iter().zip(values) {
            let (source, dest) = (node[0], node[1]);
            match graph.get_mut(source) {
                Some(x) => x.push((dest.to_owned(), weight)),
                None => {
                    let v = vec![(dest.to_owned(), weight)];
                    graph.insert(source.to_owned(), v);
                }
            }

            match graph.get_mut(dest) {
                Some(x) => x.push((source.to_owned(), (1.0 / weight))),
                None => {
                    let v = vec![(source.to_owned(), 1.0 / weight)];
                    graph.insert(dest.to_owned(), v);
                }
            }
        }
        
        let mut ans: Vec<f32> = Vec::new();
        for edge in queries.iter() {
            let (dividend, divisor) = (edge[0], edge[1]);
            if !graph.contains_key(dividend) || !graph.contains_key(divisor) {
                ans.push(-1.0);
            }
            else if dividend == divisor {
                ans.push(1.0);
            } 
            else {
                let res = Solution::bfs(&graph, dividend, divisor);
                ans.push(res);
            }
        }
            
        return ans

    }
    
    #[allow(dead_code)]
    fn bfs(graph: &Graph, source: &str, target: &str) -> f32 {
        let mut __q: VecDeque<(&str, f32)> = VecDeque::new();
        let mut visited: HashSet<&str> = HashSet::new();
        __q.push_back((source, 1.0));
        visited.insert(source);
        while !__q.is_empty() {
            let (node, prod) = __q.pop_front().unwrap();
            // If target reached
            if node == target {
                return prod;
            }
            
            // For each divisor and quotient
            let childrens = graph.get(node.into()).unwrap().iter();
            for (ch, value) in childrens {
                if !visited.contains(ch.as_str()) {
                    visited.insert(ch.as_str());
                    __q.push_back((ch, value * prod));
                }
            }

        }
        
        -1.0

    }
    
}

#[cfg(test)]
mod test {
    use super::Solution;
    
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
        println!(">>> Executing Test-3 >>>");
        let equations = vec![["a", "b"]];
        let values = vec![0.5];
        let queries = vec![["a", "b"], ["b", "a"], ["a", "c"], ["x", "y"]];
        let output = vec![0.50000, 2.00000, -1.00000, -1.00000];
        let ans = Solution::calc_equation(equations, values, queries);
        assert_eq!(ans, output);
    }
    
}


#![allow(unused)]
use std::{collections::HashMap, borrow::BorrowMut};

struct Solution;
type Graph = HashMap<i32, Vec<i32>>;


impl Solution {
    /// 207. Course Schedule
    /// https://leetcode.com/problems/course-schedule/description/
    ///
    /// There are a total of numCourses courses you have to take, labeled from 0 to
    /// numCourses - 1. You are given an array prerequisites where prerequisites[i] = [ai, bi]
    /// indicates that you must take course bi first if you want to take course ai.
    ///
    /// For example, the pair [0, 1], indicates that to take course 0 you have to first take
    /// course 1.
    ///
    /// Return true if you can finish all courses. Otherwise, return false.
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let numCourses = 2; let prerequisites = vec![vec![1,0]];
    /// assert_eq!(Solution::can_finish(numCourses, prerequisites), true);
    /// ``` 
    /// Explanation: There are a total of 2 courses to take.
    /// To take course 1 you should have finished course 0. So it is possible.
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let numCourses = 2; let prerequisites = vec![vec![1,0],vec![0,1]];
    /// assert_eq!(Solution::can_finish(numCourses, prerequisites), false);
    /// ```
    /// Explanation: There are a total of 2 courses to take. 
    /// To take course 1 you should have finished course 0, and to take course 0 you should
    /// also have finished course 1. So it is impossible.
    ///
    /// Constraints:
    /// ------------
    /// * 1 <= numCourses <= 2000
    /// * 0 <= prerequisites.length <= 5000
    /// * prerequisites[i].length == 2
    /// * 0 <= ai, bi < numCourses
    /// 
    /// All the pairs prerequisites[i] are unique.
    ///
    fn can_finish(num_courses: i32, prerequisites: Vec<[i32; 2]>) -> bool {
        // Build graph and dfs
        let mut visited = vec![0; num_courses as usize];
        let mut graph: Graph = HashMap::new();
        for preq in prerequisites {
            let (n1, n2) = (preq[0], preq[1]);
            match graph.get_mut(&n1) {
                Some(node) =>  { 
                    node.push(n2);
                },
                None => { graph.insert(n1, vec![n2]); }
            }
        }

        // Iterate for every disjoint node
        for i in 0..num_courses {
            if visited[i as usize] == 0 && !Solution::dfs(
                i as usize, &graph, visited.borrow_mut()) {
                return false;
            }
        }

        true
    }
    
    /// - For dfs Mark node visited[node] = -1 to look for cycle
    /// - Time complexity: O(V + E)  # For directed edge
    fn dfs(node: usize, graph: &Graph, visited: &mut Vec<i32>) -> bool {
        // If cycle detected
        if !visited.is_empty() && visited[node] == -1 {
            return false;
        }
       
        // if it is done visted, then do not visit again
        if !visited.is_empty() && visited[node] == 1 {
            return true;
        }
       
        visited[node] = -1;
        if let Some(children) = graph.get(&(node as i32)) {
            for child in children {
                if !Solution::dfs(*child as usize, graph, visited) {
                    return false;
                }
            } 
        }
        visited[node] = 1;
        true
    }


}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let num_courses = 2;
        let prerequisites = vec![[1, 0]];
        let ans = Solution::can_finish(num_courses, prerequisites);
        assert_eq!(ans, true);
    } 
    
    #[test]
    fn test2() {
        let num_courses = 2;
        let prerequisites = vec![[1, 0], [0, 1]];
        let ans = Solution::can_finish(num_courses, prerequisites);
        assert_eq!(ans, false);
    } 
    
}


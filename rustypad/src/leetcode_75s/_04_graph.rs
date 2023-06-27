#![allow(unused)]
use std::{collections::{HashSet, HashMap, VecDeque}, cell::RefCell, rc::Rc, hash::Hash};

struct Solution;

#[derive(Debug, Clone)]
struct Node {
    val: i32,
    neighbors: Vec<Option<NodeRef>>
}

type NodeRef = Rc<RefCell<Node>>;


impl Solution {
    /// ## 200. Number of Islands
    /// https://leetcode.com/problems/number-of-islands/
    /// 
    /// Given an m x n 2D binary grid grid which represents a map of '1's (land) and '0's (water), 
    /// return the number of islands.
    ///
    /// An island is surrounded by water and is formed by connecting adjacent lands horizontally or 
    /// vertically. You may assume all four edges of the grid are all surrounded by water.
    ///
    /// Example 1:
    /// ----------
    /// Input: grid = [
    /// ["1","1","1","1","0"],
    /// ["1","1","0","1","0"],
    /// ["1","1","0","0","0"],
    /// ["0","0","0","0","0"]
    /// ]
    /// 
    /// Output: 1
    /// 
    /// Example 2:
    ///
    /// Input: grid = [
    /// ["1","1","0","0","0"],
    /// ["1","1","0","0","0"],
    /// ["0","0","1","0","0"],
    /// ["0","0","0","1","1"]
    /// ]
    /// 
    /// Output: 3
    ///
    /// Constraints:
    /// -----------
    /// m == grid.length
    /// n == grid[i].length
    /// 1 <= m, n <= 300
    /// grid[i][j] is '0' or '1'.
    ///
    // Save coordinates in HashSet
    // Visit each land using dfs and mark it as visited
    // Return number of islands
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        
        fn direction(row: i32, col: i32, grid: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
            let (n_rows, n_cols) = (grid.len() as i32, grid[0].len() as i32);
            let mut ans: Vec<(i32, i32)> = vec![];
            let directions: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
            for (r, c) in directions {
                if (r + row  >= 0 && row + r < n_rows) && 
                (col + c >= 0 && col + c < n_cols) &&
                grid[(r + row) as usize][(c + col) as usize] == '1' {
                    ans.push((row + r, col + c));
                }
            } 
            ans
        }

        fn dfs(
            row: usize, col: usize, 
            grid: &Vec<Vec<char>>, visited: &mut HashSet<(usize, usize)>
        ) {
            if visited.contains(&(row, col)) {
                return
            }
            // Visit each land section using dfs and mark it as visited
            visited.insert((row, col));
            
            for (x, y) in direction(row as i32, col as i32, &grid) {
                let (x, y) = (x as usize, y as usize);
                if !visited.contains(&(x, y)) {
                    dfs(x, y, grid, visited);
                }
            }
        }

        let mut no_of_islands: i32 = 0;
        // This will save the visited coordinates
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == '1' && !visited.contains(&(r, c)) {
                    no_of_islands += 1;
                    dfs(r, c, &grid, &mut visited);
                }
            }
        }

        no_of_islands
    }

    /// Course Schedule II
    /// https://leetcode.com/problems/course-schedule-ii/description/
    ///
    /// There are a total of numCourses courses you have to take, labeled from 0 to 
    /// numCourses - 1. You are given an array prerequisites where prerequisites[i] = [a_i, b_i] 
    /// indicates that you must take course b_i first if you want to take course a_i.
    ///
    /// For example, the pair [0, 1], indicates that to take course 0 you have to first take 
    /// course 1.
    ///
    /// Return the ordering of courses you should take to finish all courses. If there are many 
    /// valid answers, return any of them. If it is impossible to finish all courses, return an 
    /// empty array.
    ///
    /// Example 1:
    /// ----------
    /// - Input: numCourses = 2, prerequisites = [[1,0]]
    /// - Output: [0,1]
    /// - Explanation: There are a total of 2 courses to take. To take course 1 you should have 
    /// finished course 0. So the correct course order is [0,1].
    ///
    /// Example 2:
    /// ----------
    /// - Input: numCourses = 4, prerequisites = [[1,0],[2,0],[3,1],[3,2]]
    /// - Output: [0,2,1,3]
    /// - Explanation: There are a total of 4 courses to take. To take course 3 you should have 
    /// finished both courses 1 and 2. Both courses 1 and 2 should be taken after you finished 
    /// course 0.
    ///
    /// So one correct course order is [0,1,2,3]. Another correct ordering is [0,2,1,3].
    ///
    /// Example 3:
    /// ----------
    /// - Input: numCourses = 1, prerequisites = []
    /// - Output: [0]
    ///
    /// Constraints:
    /// ------------
    /// - 1 <= numCourses <= 2000
    /// - 0 <= prerequisites.length <= numCourses * (numCourses - 1)
    /// - prerequisites[i].length == 2
    ///
    /// ALGO
    /// - build graph and set indegree count for each node
    /// - Start with node with 0 indegree and visit all neighbour and reduce 
    /// in-degree count by -1
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        unimplemented!()       
    }

    /// ## 207. Course Schedule
    ///  https://leetcode.com/problems/course-schedule/
    ///
    /// There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. 
    /// You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you 
    /// must take course bi first if you want to take course ai.
    ///
    /// For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.
    /// Return true if you can finish all courses. Otherwise, return false.
    ///
    /// Example 1:
    /// -----------
    /// ```
    /// let numCourses = 2; let prerequisites = vec![vec![1,0]];
    /// assert_eq!(Solution::can_finish(numCourses, prerequisites), true);
    /// ```
    /// - Explanation: There are a total of 2 courses to take. 
    /// To take course 1 you should have finished course 0. So it is possible.
    /// 
    /// Example 2:
    /// ----------
    /// ```
    /// let numCourses = 2; let prerequisites = vec![vec![1,0],vec![0,1]];
    /// assert_eq!(Solution::can_finish(numCourses, prerequisites), false);
    /// ```
    /// - Explanation: There are a total of 2 courses to take. 
    /// To take course 1 you should have finished course 0, and to take course 0 you should also have 
    /// finished course 1. So it is impossible.
    ///
    /// Constraints:
    /// -----------
    /// - 1 <= numCourses <= 2000
    /// - 0 <= prerequisites.length <= 5000
    /// - prerequisites[i].length == 2
    /// - 0 <= ai, bi < numCourses
    ///
    /// All the pairs prerequisites[i] are unique.
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut indegree: Vec<i32> = vec![0; num_courses as usize];

        // Add indegree count for node with indegree and build graph here
        for courses in prerequisites {
            graph.entry(courses[1]).or_insert(vec![]).push(courses[0]);
            indegree[courses[0] as usize] += 1;
        }

        let mut deque: VecDeque<i32> = VecDeque::new();
        for node in 0..num_courses {
            if indegree[node as usize] == 0 {
                deque.push_back(node.clone());
            }
        }
        let mut node_visited = 0;
        while !deque.is_empty() {
            let node = deque.pop_front().unwrap();
            node_visited += 1;
            
            while let Some(children) = graph.get(&node) {
                for childnode in children {
                    indegree[*childnode as usize] -= 1;
                    if indegree[*childnode as usize] == 0 {
                        deque.push_back(childnode.clone());
                    }
                }
                break;
            }
        }

        node_visited == num_courses
    }

    /// ## 133. Clone Graph
    /// https://leetcode.com/problems/clone-graph/
    ///
    /// Given a reference of a node in a connected undirected graph.
    ///
    /// Return a deep copy (clone) of the graph.
    ///
    /// Each node in the graph contains a value (int) and a list (List[Node]) of its neighbors.
    /// ```
    /// struct Node {
    ///     val: i32;
    ///     neighbors: Vec<Node>;
    /// }
    /// ```
    /// Test case format:
    ///
    /// For simplicity, each node's value is the same as the node's index (1-indexed). For example, 
    /// the first node with val == 1, the second node with val == 2, and so on. The graph is represented 
    /// in the test case using an adjacency list.
    ///
    /// An adjacency list is a collection of unordered lists used to represent a finite graph. Each list 
    /// describes the set of neighbors of a node in the graph.
    ///
    /// The given node will always be the first node with val = 1. You must return the copy of the given 
    /// node as a reference to the cloned graph.
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let adjList = vec![vec![2,4],vec![1,3],vec![2,4],vec![1,3]];
    /// let res = vec![vec![2,4],vec![1,3],vec![2,4],vec![1,3]];
    /// assert_eq!(Solution::clone_graph(to_graph(adjList)), to_graph(res));
    /// ```
    /// Explanation: There are 4 nodes in the graph.
    /// 1st node (val = 1)'s neighbors are 2nd node (val = 2) and 4th node (val = 4).
    /// 2nd node (val = 2)'s neighbors are 1st node (val = 1) and 3rd node (val = 3).
    /// 3rd node (val = 3)'s neighbors are 2nd node (val = 2) and 4th node (val = 4).
    /// 4th node (val = 4)'s neighbors are 1st node (val = 1) and 3rd node (val = 3).
    ///
    /// Example 2:
    /// -----------
    /// ```
    /// let adjList = vec![vec![]];
    /// assert_eq!(Solution::clone_graph(to_graph(adjList)), to_graph(vec![vec![]]));
    /// ```
    /// Explanation: Note that the input contains one empty list. The graph consists of 
    /// only one node with val = 1 and it does not have any neighbors.
    ///
    /// Example 3:
    /// -----------
    /// ```
    /// let adjList = vec![];
    /// assert_eq!(Solution::clone_graph(to_graph(adjList)), to_graph(vec![]));
    /// ```
    /// Explanation: This an empty graph, it does not have any nodes.
    ///
    /// Constraints:
    /// -----------
    /// The number of nodes in the graph is in the range [0, 100].
    /// * 1 <= Node.val <= 100
    /// * Node.val is unique for each node.
    /// * There are no repeated edges and no self-loops in the graph.
    /// * The Graph is connected and all nodes can be visited starting from the given node.
    fn clone_graph(node: Option<NodeRef>) -> Option<NodeRef> {
        // DFS to node and return clone
        fn clone(node: Option<NodeRef>, cache: &mut HashMap<i32, NodeRef>) -> Option<NodeRef> {
            
            if let Some(node) = node {
                // If node is already cloned
                if let Some(node) = cache.get(&node.borrow().val) {
                    return Some(node.clone());
                }
                
                // Create a clone node
                let clone_node = Rc::new(RefCell::new(
                    Node {
                        val: node.borrow().val,
                        neighbors: Vec::new()
                    }
                ));
                // Insert the clone node to hashmap
                cache.insert(node.borrow().val, clone_node.clone());

                for child in node.borrow().neighbors.iter() {
                    clone_node.borrow_mut().neighbors.push(clone(child.clone(), cache));
                }
                
                return Some(clone_node);
            }

            None
        }
        
        let mut cache: HashMap<i32, NodeRef> = HashMap::new();
        clone(node, &mut cache)

    }

}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_islands() {
        let grid = vec![
            vec!['1','1','1','1','0'],
            vec!['1','1','0','1','0'],
            vec!['1','1','0','0','0'],
            vec!['0','0','0','0','0']
        ];
    
        assert_eq!(Solution::num_islands(grid), 1);

        let grid = vec![
            vec!['1','1','0','0','0'],
            vec!['1','1','0','0','0'],
            vec!['0','0','1','0','0'],
            vec!['0','0','0','1','1']
        ];
        assert_eq!(Solution::num_islands(grid), 3);
    }

    #[test]
    fn test_finish() {
        fn test1() {
            let num_courses = 2;
            let prerequisites = vec![vec![1, 0]];
            let ans = Solution::can_finish(num_courses, prerequisites);
            assert_eq!(ans, true);
        } 
        
        fn test2() {
            let num_courses = 2;
            let prerequisites = vec![vec![1, 0], vec![0, 1]];
            let ans = Solution::can_finish(num_courses, prerequisites);
            assert_eq!(ans, false);
        }

        test1();
        test2();
    }
}
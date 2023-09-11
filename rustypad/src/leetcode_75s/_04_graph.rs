#![allow(unused)]
use std::{collections::{HashSet, HashMap, VecDeque}, cell::RefCell, rc::Rc, hash::Hash};
use std::error::Error;

struct Solution;

#[derive(Debug, Clone)]
struct Node {
    val: i32,
    neighbors: Vec<Option<NodeRef>>
}

type NodeRef = Rc<RefCell<Node>>;

struct UnionFind {
    pub n: i32,
    root: Vec<i32>
}

impl UnionFind {
    fn new(n: i32) -> Self {
        UnionFind {
            n,
            root: (0..n).collect()
        }
    }

    fn find(&mut self, x: i32) -> i32 {
        // If the parent of node is itself then return it
        if self.root[x as usize] == x {
            return x;
        }
        self.root[x as usize] = self.find(self.root[x as usize]);
        self.root[x as usize]
    }

    fn union(&mut self, x: i32, y: i32) {
        let x1 = self.find(x);
        let y1 = self.find(y);
        // If x and y are in the different sets, then union them
        if x1 != y1 {
            self.root[x1 as usize] = y1;
            self.n -= 1;
        }
    }
}
impl Solution {
    /// ## Number of Connected Components in an Undirected Graph
    /// https://leetcode.com/problems/number-of-connected-components-in-an-undirected-graph/description/
    ///
    ///
    /// You have a graph of n nodes. You are given an integer n and an array edges 
    /// where edges[i] = [ai, bi] indicates that there is an edge between ai and bi in the graph.
    ///
    /// Return the number of connected components in the graph.
    ///
    /// Example 1:
    /// ----------
    /// ```doc
    /// 0 ---1      3
    ///      |      |
    ///      2      4
    /// ```
    /// ```
    /// let n = 5; let edges = vec![vec![0,1],vec![1,2],vec![3,4]];
    /// assert_eq!(Solution::count_components(n, edges), 2);
    /// ```
    ///
    /// Example 2:
    /// ----------
    /// ```doc
    /// 0 --1   3
    ///     | / |
    ///     2   4
    /// ```
    /// ```
    /// let n = 5; let edges = vec![vec![0,1],vec![1,2],vec![2,3],vec![3,4]];
    /// assert_eq!(Solution::count_components(n, edges), 1);
    /// ```
    /// Constraints:
    /// -----------
    /// * 1 <= n <= 2000
    /// * 1 <= edges.length <= 5000
    /// * edges[i].length == 2
    /// * 0 <= ai <= bi < n
    /// * ai != bi
    /// * There are no repeated edges.
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut uf = UnionFind::new(n);
        for edge in edges {
            uf.union(edge[0], edge[1]);
        }
        uf.n
    }

    /// ## 261. Graph Valid Tree
    /// https://leetcode.com/problems/graph-valid-tree/editorial/
    /// 
    /// You have a graph of n nodes labeled from 0 to n - 1. You are given an integer n and a list of 
    /// edges where edges[i] = [ai, bi] indicates that there is an undirected edge between nodes ai and bi in the graph.
    ///
    /// Return true if the edges of the given graph make up a valid tree, and false otherwise.
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let n = 5; let edges = vec![vec![0,1],vec![0,2],vec![0,3],vec![1,4]];
    /// assert_eq!(Solution::valid_tree(n, edges), true);
    /// ```
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let n = 5; let edges = vec![vec![0,1],vec![1,2],vec![2,3],vec![1,3],vec![1,4]];
    /// assert_eq!(Solution::valid_tree(n, edges), false);
    /// ```
    ///
    /// Constraints:
    /// -----------
    /// * 1 <= n <= 2000
    /// * 0 <= edges.length <= 5000
    /// * edges[i].length == 2
    /// * 0 <= ai, bi < n
    /// * ai != bi
    /// * There are no self-loops or repeated edges.
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        // For a graph to be valid tree there should be exactly n-1 edges
        if edges.len() as i32 !=  n-1 {
            return false;
        }

        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        for edge in edges {
            graph.entry(edge[0]).or_insert(vec![]).push(edge[1]);
            graph.entry(edge[1]).or_insert(vec![]).push(edge[0]);
        }

        let mut visited: HashSet<i32> = HashSet::new();
        let mut stack: Vec<i32> = vec![0];

        // DFS 
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            visited.insert(node);
            if !graph.contains_key(&node) {
                continue;
            }

            for child in graph.get(&node).unwrap() {
                if !visited.contains(child) {
                    stack.push(*child);
                }
            }
        }

        visited.len() as i32 == n
    }

    /// ### Longest Consecutive Sequence
    /// https://leetcode.com/problems/longest-consecutive-sequence/
    ///
    /// Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.
    ///
    /// You must write an algorithm that runs in O(n) time.
    ///
    /// Example 1:
    // ----------
    /// ```
    /// let nums = vec![100,4,200,1,3,2];
    /// assert_eq!(Solution::longest_consecutive(nums), 4);
    /// ```
    /// *Explanation*: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let nums = vec![0,3,7,2,5,8,4,6,0,1];
    /// assert_eq!(Solution::longest_consecutive(nums), 9);
    /// ```
    /// Constraints:
    /// -----------
    /// - 0 <= nums.length <= 105
    /// - -109 <= nums[i] <= 109
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        unimplemented!()    
    }

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
    /// ```
    /// let numCourses = 2; let prerequisites = vec![vec![1,0]];
    /// assert_eq!(Solution::find_order(numCourses, prerequisites), vec![0,1]);
    /// ```
    /// - Explanation: There are a total of 2 courses to take. To take course 1 you should have 
    /// finished course 0. So the correct course order is [0,1].
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let numCourses = 4; let prerequisites = vec![vec![1,0],vec![2,0],vec![3,1],vec![3,2]];
    /// assert_eq!(Solution::find_order(numCourses, prerequisites), vec![0,2,1,3]);
    /// ```
    /// - Explanation: There are a total of 4 courses to take. To take course 3 you should have 
    /// finished both courses 1 and 2. Both courses 1 and 2 should be taken after you finished 
    /// course 0.
    ///
    /// So one correct course order is [0,1,2,3]. Another correct ordering is [0,2,1,3].
    ///
    /// Example 3:
    /// ----------
    /// ```
    /// let numCourses = 1; let prerequisites = vec![];
    /// assert_eq!(Solution::find_order(numCourses, prerequisites), vec![0]);
    /// ```
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
        let mut ans: Vec<i32> = vec![];
        // Create a indegree dictionary to cache each vertex indegree count
        let mut indegree: Vec<i32> = vec![0; num_courses as usize];
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut deque: VecDeque<i32> = VecDeque::new();

        // Build graph from prerequisites vec!
        for pre in prerequisites {
            graph.entry(pre[1]).or_insert(vec![]).push(pre[0]);
            indegree[pre[0] as usize] += 1;
        }

        // For topological order use deque that start with vertex with indegree 0
        for i in 0..num_courses {
            if indegree[i as usize] == 0 {
                deque.push_back(i);
            }
        }

        while !deque.is_empty() {
            let node = deque.pop_front().unwrap();
            ans.push(node.clone());
            // Get all children of that node and decrement the indegree count of children
            if let Some(conn) = graph.get(&node) {
                for &children in conn {
                    indegree[children as usize] -= 1;
                    if indegree[children as usize] == 0 {
                        deque.push_back(children.clone());
                    }
                }
            }
        }

        if ans.len() as i32 == num_courses {ans} else {vec![]} 
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
            
            match node {
                Some(node) => {
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
                _ => None,
            }
        }
        
        let mut cache: HashMap<i32, NodeRef> = HashMap::new();
        clone(node, &mut cache)

    }

    /// ## Jump Game III
    ///
    /// Given an array of non-negative integers arr, you are initially positioned at start index of the array. 
    /// When you are at index i, you can jump to i + arr[i] or i - arr[i], check if you can reach to any index 
    /// with value 0.
    ///
    /// Notice that you can not jump outside of the array at any time.
    ///
    /// Example 1:
    /// ----------
    /// Input: arr = [4,2,3,0,3,1,2], start = 5
    /// 
    /// Output: true
    /// 
    /// Explanation: 
    /// All possible ways to reach at index 3 with value 0 are: 
    /// index 5 -> index 4 -> index 1 -> index 3 
    /// index 5 -> index 6 -> index 4 -> index 1 -> index 3 
    ///
    /// Example 2:
    /// ----------
    /// Input: arr = [4,2,3,0,3,1,2], start = 0
    /// 
    /// Output: true 
    /// 
    /// Explanation: 
    /// One possible way to reach at index 3 with value 0 is: 
    /// index 0 -> index 4 -> index 1 -> index 3
    /// 
    /// Example 3:
    /// ----------
    /// Input: arr = [3,0,2,1,2], start = 2
    /// 
    /// Output: false
    /// 
    /// Explanation: There is no way to reach at index 1 with value 0.
    ///
    /// Constraints:
    /// ----------
    /// 1 <= arr.length <= 5 * 104
    /// 0 <= arr[i] < arr.length
    /// 0 <= start < arr.length
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        fn get_connections(index: i32, arr: &Vec<i32>) -> Vec<i32> {
            let mut pos: Vec<i32> = Vec::new();
            if index + arr[index as usize] < arr.len() as i32 {
                pos.push(index + arr[index as usize]);
            }
            if index - arr[index as usize] >= 0 {
                pos.push(index - arr[index as usize]);
            }
            pos
        }

        let mut deq: VecDeque<i32> = VecDeque::new();
        let mut visited: HashSet<i32> = HashSet::new();
        deq.push_back(start);
        visited.insert(start);
        
        while !deq.is_empty() {
            let idx = deq.pop_front().unwrap();
            if arr[idx as usize] == 0 {
                return true;
            }
            for ni in get_connections(idx, &arr) {
                if visited.contains(&ni) {
                    continue;
                }

                visited.insert(ni);
                deq.push_back(ni);
            }
        }

        false
    }

    /// ## 417. Pacific Atlantic Water Flow
    /// https://leetcode.com/problems/pacific-atlantic-water-flow/
    ///
    /// There is an m x n rectangular island that borders both the Pacific Ocean and Atlantic Ocean. The Pacific Ocean 
    /// touches the island's left and top edges, and the Atlantic Ocean touches the island's right and bottom edges.
    ///
    /// The island is partitioned into a grid of square cells. You are given an m x n integer matrix heights where 
    /// heights[r][c] represents the height above sea level of the cell at coordinate (r, c).
    ///
    /// The island receives a lot of rain, and the rain water can flow to neighboring cells directly north, south, east, 
    /// and west if the neighboring cell's height is less than or equal to the current cell's height. Water can flow from 
    /// any cell adjacent to an ocean into the ocean.
    ///
    /// Return a 2D list of grid coordinates result where result[i] = [ri, ci] denotes that rain water can flow from cell 
    /// (ri, ci) to both the Pacific and Atlantic oceans.
    ///
    ///
    ///
    /// Example 1:
    /// ---------
    /// ```
    /// let heights = vec![vec![1,2,2,3,5],vec![3,2,3,4,4],vec![2,4,5,3,1],vec![6,7,1,4,5],vec![5,1,1,2,4]];
    /// let ans = vec![[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]];
    /// assert_eq!(Solution::pacific_atlantic(heights), ans);
    /// ```
    /// 
    /// ```doc
    /// Explanation: The following cells can flow to the Pacific and Atlantic oceans, as shown below:
    /// [0,4]: [0,4] -> Pacific Ocean 
    /// [0,4] -> Atlantic Ocean
    /// [1,3]: [1,3] -> [0,3] -> Pacific Ocean 
    /// [1,3] -> [1,4] -> Atlantic Ocean
    /// [1,4]: [1,4] -> [1,3] -> [0,3] -> Pacific Ocean 
    /// [1,4] -> Atlantic Ocean
    /// [2,2]: [2,2] -> [1,2] -> [0,2] -> Pacific Ocean 
    /// [2,2] -> [2,3] -> [2,4] -> Atlantic Ocean
    /// [3,0]: [3,0] -> Pacific Ocean 
    /// [3,0] -> [4,0] -> Atlantic Ocean
    /// [3,1]: [3,1] -> [3,0] -> Pacific Ocean 
    /// [3,1] -> [4,1] -> Atlantic Ocean
    /// [4,0]: [4,0] -> Pacific Ocean 
    /// [4,0] -> Atlantic Ocean
    /// Note that there are other possible paths for these cells to flow to the Pacific and Atlantic oceans.
    /// ```
    /// 
    /// Example 2:
    /// ---------
    /// ```
    /// let heights = vec![vec![1]];
    /// let ans = vec![vec![0,0]]
    /// assert_eq!(Solution::pacific_atlantic(heights), ans);
    /// ```
    /// Explanation: The water can flow from the only cell to the Pacific and Atlantic oceans.
    ///
    ///
    /// Constraints:
    /// ----------------
    /// - m == heights.length
    /// - n == heights[r].length
    /// - 1 <= m, n <= 200
    /// - 0 <= heights[r][c] <= 105
    
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn bfs_traversal(
            heights: &Vec<Vec<i32>>, deq: &mut VecDeque<(i32, i32)>
        ) -> HashSet<(i32, i32)> {
            let directions  = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
            let mut visited: HashSet<(i32, i32)> = HashSet::new();
            
            while !deq.is_empty() {
                let (row, col) = deq.pop_front().unwrap();
                visited.insert((row, col));

                for (dx, dy) in &directions {
                    let (new_x, new_y) = (row + *dx, col + *dy);
                    if new_x >= heights.len() as i32 && new_x < 0 && 
                        new_y >= heights[0].len() as i32 && new_y < 0 {
                        continue;
                    }
                    if visited.contains(&(new_x, new_y)) {
                        continue;
                    }
                    if heights[new_x as usize][new_y as usize] < heights[row as usize][col as usize]{
                        continue;
                    }
                    deq.push_back((new_x, new_y));
                }
            }
        
            visited
        }

        let mut pacific_deq: VecDeque<(i32, i32)> = VecDeque::new();
        let mut atlantic_deq: VecDeque<(i32, i32)> = VecDeque::new();
        let (rows, cols) = (heights.len(), heights[0].len());

        // Start from coastal land of both pacific and atlantic island
        for i in 0..rows {
            pacific_deq.push_back((i as i32, 0));
            atlantic_deq.push_back((i as i32, cols as i32 - 1));
        }

        for i in 0..cols {
            pacific_deq.push_back((0, i as i32));
            atlantic_deq.push_back(((rows- i) as i32, i as i32));
        }

        let visited_atlantic = bfs_traversal(&heights, &mut atlantic_deq);
        let visited_pacific = bfs_traversal(&heights, &mut pacific_deq);

        let ans = visited_atlantic.intersection(&visited_pacific)
            .map(|&x| vec![x.0, x.1]).collect();
        ans
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pacific_atlantic() {
        // Test case 1
        let heights = vec![vec![1,2,2,3,5],vec![3,2,3,4,4],vec![2,4,5,3,1],vec![6,7,1,4,5],vec![5,1,1,2,4]];
        let ans = vec![[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]];
        assert_eq!(Solution::pacific_atlantic(heights), ans);

        // Test case 2
        let heights = vec![vec![1]];
        let ans = vec![vec![0,0]];
        assert_eq!(Solution::pacific_atlantic(heights), ans);   
    }

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

    #[test]
    fn test_find_order() {
        let num_courses = 2; let prerequisites = vec![vec![1,0]];
        assert_eq!(Solution::find_order(num_courses, prerequisites), vec![0,1]);

        let num_courses = 4; let prerequisites = vec![vec![1,0],vec![2,0],vec![3,1],vec![3,2]];
        assert_eq!(Solution::find_order(num_courses, prerequisites), vec![0,1,2,3]);
        
        let num_courses = 1; let prerequisites = vec![];
        assert_eq!(Solution::find_order(num_courses, prerequisites), vec![0]);
    }

    #[test]
    fn test_can_reach() {
        let arr = vec![4,2,3,0,3,1,2]; let start = 5;
        assert_eq!(Solution::can_reach(arr, start), true);
        
        let arr = vec![4,2,3,0,3,1,2]; let start = 0;
        assert_eq!(Solution::can_reach(arr, start), true);
    
        let arr = vec![3,0,2,1,2]; let start = 2;
        assert_eq!(Solution::can_reach(arr, start), false);
    }
}
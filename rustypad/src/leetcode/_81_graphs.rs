#![allow(unused)]
use std::{collections::{HashMap, HashSet, VecDeque}, hash::Hash};


#[derive(Debug, PartialEq, Eq)]
struct Solution;

type Graph = HashMap<i32, Vec<i32>>;

impl Solution {
    /// ## 547. Number of Provinces
    ///
    /// There are n cities. Some of them are connected, while some are not. If city a is connected directly 
    /// with city b, and city b is connected directly with city c, then city a is connected indirectly with city c.
    ///
    /// A province is a group of directly or indirectly connected cities and no other cities outside of the group.
    ///
    /// You are given an n x n matrix isConnected where isConnected[i][j] = 1 if the ith city and the jth city are 
    /// directly connected, and isConnected[i][j] = 0 otherwise.
    ///
    /// Return the total number of provinces.
    ///
    /// Example 1:
    /// ----------
    /// Input: isConnected = [[1,1,0],[1,1,0],[0,0,1]]
    /// Output: 2
    /// 
    /// Example 2:
    /// ----------
    /// Input: isConnected = [[1,0,0],[0,1,0],[0,0,1]]
    /// Output: 3
    ///
    ///
    // Constraints:
    /// -----------
    /// 1 <= n <= 200
    /// n == isConnected.length
    /// n == isConnected[i].length
    /// isConnected[i][j] is 1 or 0.
    /// isConnected[i][i] == 1
    /// isConnected[i][j] == isConnected[j][i]
    ///
    ///
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        fn dfs(node: i32, graph: &Graph, visited: &mut HashSet<i32>) {
            if let Some(childrens) = graph.get(&node) {
                visited.insert(node);
                for child in childrens {
                    if !visited.contains(child) {
                        dfs(*child, graph, visited);
                    } 
                }
            }
        }

        let mut graph: Graph = HashMap::new();
        let mut visited: HashSet<i32> = HashSet::new();

        // Build graph from upper tringle matrix
        for row in 0..is_connected.len() {
            for col in (row+1)..is_connected.len() {
                
                if is_connected[row][col] == 1 {
                    // Entry source to dest mapping
                    let entry = graph.entry(row as i32).or_insert(vec![]);
                    entry.push(col as i32);
                    
                    // Entry dest to source mapping
                    graph.entry(col as i32).or_insert(vec![]).push(row as i32);
                }
            }
        }

        let mut node_count: i32 = graph.keys().len() as i32;
        let mut ans: i32 = 0;
        for i in 0..is_connected.len() {
            if !visited.contains(&(i as i32)) {
                ans += 1;
                dfs(i as i32, &graph, &mut visited);
                visited.insert(i as i32);
            }
        }

        ans
    }

    /// ## 1466. Reorder Routes to Make All Paths Lead to the City Zero
    /// https://leetcode.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero/
    ///
    /// There are n cities numbered from 0 to n - 1 and n - 1 roads such that there is only one way to 
    /// travel between two different cities (this network form a tree). Last year, The ministry of transport 
    /// decided to orient the roads in one direction because they are too narrow.
    ///
    /// Roads are represented by connections where connections[i] = [ai, bi] represents a road from city ai 
    /// to city bi.
    ///
    /// This year, there will be a big event in the capital (city 0), and many people want to travel to this city.
    ///
    /// Your task consists of reorienting some roads such that each city can visit the city 0. Return the minimum 
    /// number of edges changed.
    ///
    /// It's guaranteed that each city can reach city 0 after reorder.
    ///
    /// Example 1:
    /// ----------
    /// Input: n = 6, connections = [[0,1],[1,3],[2,3],[4,0],[4,5]]
    /// Output: 3
    /// Explanation: Change the direction of edges show in red such that each node can reach the node 0 (capital).
    ///
    /// Example 2:
    /// ----------
    /// Input: n = 5, connections = [[1,0],[1,2],[3,2],[3,4]]
    /// Output: 2
    /// Explanation: Change the direction of edges show in red such that each node can reach the node 0 (capital).
    ///
    /// Example 3:
    /// ----------
    /// Input: n = 3, connections = [[1,0],[2,0]]
    /// Output: 0
    ///
    /// Constraints:
    /// ------------
    /// 2 <= n <= 5 * 104
    /// connections.length == n - 1
    /// connections[i].length == 2
    /// 0 <= ai, bi <= n - 1
    /// ai != bi
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        fn dfs(
            source: i32, 
            graph: &HashMap<i32, Vec<i32>>, 
            edges: &HashSet<(i32, i32)>, visited: &mut HashSet<i32>
        ) -> i32 {
            let mut ans: i32 = 0;
            visited.insert(source);

            for node in graph.get(&source).unwrap().iter() {
                if !visited.contains(node) {
                    if edges.contains(&(source, *node)) {
                        ans += 1
                    }
    
                    ans += dfs(*node, graph, edges, visited);    
                }
                
            }

            ans
        }

        let mut edges: HashSet<(i32, i32)> = HashSet::new();
        let mut visited: HashSet<i32> = HashSet::new();
        // Since there are atmost n - 1 edges
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        for edge in connections {
            let entry: &mut Vec<i32> = graph.entry(edge[0]).or_insert(vec![]);
            entry.push(edge[1]);
            
            let entry: &mut Vec<i32> = graph.entry(edge[1]).or_insert(vec![]);
            entry.push(edge[0]);
            
            // Insert the edge that need to be reversed in edges
            edges.insert((edge[0], edge[1]));
        }

        dfs(0, &graph, &edges, &mut visited)

    }

    /// ## 841. Keys and Rooms
    /// https://leetcode.com/problems/keys-and-rooms/
    ///
    /// There are n rooms labeled from 0 to n - 1 and all the rooms are locked except for room 0. 
    /// Your goal is to visit all the rooms. However, you cannot enter a locked room without having its key.
    ///
    /// When you visit a room, you may find a set of distinct keys in it. Each key has a number 
    /// on it, denoting which room it unlocks, and you can take all of them with you to unlock the other rooms.
    ///
    /// Given an array rooms where rooms[i] is the set of keys that you can obtain if you visited 
    /// room i, return true if you can visit all the rooms, or false otherwise.
    ///
    /// Example 1:
    /// ----------
    /// Input: rooms = [[1],[2],[3],[]]
    /// Output: true
    /// Explanation: 
    /// We visit room 0 and pick up key 1.
    /// We then visit room 1 and pick up key 2.
    /// We then visit room 2 and pick up key 3.
    /// We then visit room 3.
    /// Since we were able to visit every room, we return true.
    /// 
    /// Example 2:
    /// --------
    /// Input: rooms = [[1,3],[3,0,1],[2],[0]]
    /// Output: false
    /// Explanation: We can not enter room number 2 since the only key that unlocks it is in that room.
    ///
    ///
    /// Constraints:
    /// -----------
    /// n == rooms.length
    /// 2 <= n <= 1000
    /// 0 <= rooms[i].length <= 1000
    /// 1 <= sum(rooms[i].length) <= 3000
    /// 0 <= rooms[i][j] < n
    /// All the values of rooms[i] are unique.
    /// 
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        // Return a visited set of len == rooms.length
        // DFS from each room and check if it is visited 
        fn dfs(
            source: i32, 
            rooms: &Vec<Vec<i32>>, 
            visited: &mut HashSet<i32>
        ) {
            visited.insert(source);
            
            for room in rooms[source as usize].iter() {
                if !visited.contains(&room) {
                    visited.insert(*room);
                    dfs(*room, rooms, visited);
                }
            }
        }
        
        let mut visited: HashSet<i32> = HashSet::new();
            
        dfs(0, &rooms, &mut visited);
        visited.len() == rooms.len()

    }

    /// ## 1091. Shortest Path in Binary Matrix
    ///
    /// Given an n x n binary matrix grid, return the length of the shortest clear path in the matrix. 
    /// If there is no clear path, return -1.
    ///
    /// A clear path in a binary matrix is a path from the top-left cell (i.e., (0, 0)) to the bottom-right 
    /// cell (i.e., (n - 1, n - 1)) such that:
    ///
    /// All the visited cells of the path are 0.
    /// All the adjacent cells of the path are 8-directionally connected (i.e., they are different and they 
    /// share an edge or a corner). The length of a clear path is the number of visited cells of this path.
    ///
    /// Example 1:
    /// ----------
    /// Input: grid = [[0,1],[1,0]]
    /// Output: 2
    ///
    /// Example 2:
    /// ----------
    /// Input: grid = [[0,0,0],[1,1,0],[1,1,0]]
    /// Output: 4
    ///
    /// Example 3:
    /// ----------
    /// Input: grid = [[1,0,0],[1,1,0],[1,1,0]]
    /// Output: -1
    ///
    /// Constraints:
    /// ------------
    /// n == grid.length
    /// n == grid[i].length
    /// 1 <= n <= 100
    /// grid[i][j] is 0 or 1
    /// 
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        fn direction(row: i32, col: i32, grid: &Vec<Vec<i32>>) -> Vec<(i32, i32)> {
            let (n_rows, n_cols) = (grid.len() as i32, grid[0].len() as i32);
            let mut ans: Vec<(i32, i32)> = vec![];
            let directions: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];
            for (r, c) in directions {
                if (r + row  >= 0 && row + r < n_rows) && 
                (col + c >= 0 && col + c < n_cols) &&
                grid[(r + row) as usize][(c + col) as usize] == 0 {
                    ans.push((row + r, col + c));
                }
            } 
            ans
        }
        
        // If there is no path return -1
        if grid[0][0] == 1 {
            return -1;
        }

        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut deq: VecDeque<(i32, i32, i32)> = VecDeque::new();
        deq.push_back((0, 0, 1));
        visited.insert((0, 0));

        let (n_rows, n_cols) = (grid.len() as i32, grid[0].len() as i32);

        // Iterate untill deque is empty
        while !deq.is_empty() {
            let (row, col, depth) = deq.pop_front().unwrap();
            if row == n_rows -1 && col == n_cols - 1 {
                return depth
            }
            for (r, c) in direction(row, col, &grid) {
                if visited.contains(&(r, c)) {
                    continue;
                }
                visited.insert((r, c));
                deq.push_back((r, c, depth + 1));
            } 
        }
        
        -1
    }

    /// ## 42. 01 Matrix
    /// https://leetcode.com/problems/01-matrix/
    ///
    /// Given an m x n binary matrix mat, return the distance of the nearest 0 for each cell.
    ///
    /// The distance between two adjacent cells is 1.
    ///
    ///
    /// Example 1:
    /// ----------
    /// Input: mat = [[0,0,0],[0,1,0],[0,0,0]]
    /// Output: [[0,0,0],[0,1,0],[0,0,0]]
    ///
    /// Example 2:
    /// ----------
    /// Input: mat = [[0,0,0],
    ///               [0,1,0],
    ///               [1,1,1]]
    /// 
    /// Output: [[0,0,0],
    ///          [0,1,0],
    ///          [1,2,1]]
    ///
    /// Constraints:
    ///
    /// m == mat.length
    /// n == mat[i].length
    /// 1 <= m, n <= 104
    /// 1 <= m * n <= 104
    /// mat[i][j] is either 0 or 1.
    /// There is at least one 0 in mat.
    ///
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // Add all zeroes to top level in the deque
        // Pop each element from deque and add depth to next level
        // Mark the node visited if already visited

        fn directions(i: i32, j: i32, mat: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
            let mut ans: Vec<(usize, usize)> = vec![];
            let points: Vec<(i32, i32)> = vec![(0, -1), (-1, 0), (0, 1), (1, 0)];
            for (x, y) in points {
                
                if (i + x >= 0 && j + y >= 0) && 
                (i + x < mat.len() as i32 && j + y < mat[0].len() as i32) && 
                mat[(i + x) as usize][(j + y) as usize] == 1 {
                    ans.push(((i + x) as usize, (j + y) as usize));    
                }
                
            }
            ans
        }

        let mut deq: VecDeque<(i32, i32, i32)> = VecDeque::new();
        let mut ans: Vec<Vec<i32>> = vec![vec![0; mat[0].len()]; mat.len()];
        // Track the visited using HashSet
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        
        for r in 0..mat.len() {
            for c in 0..mat[0].len() {
                if mat[r][c] == 0 {
                    deq.push_back((r as i32, c as i32, 1));
                    visited.insert((r as i32, c as i32));
                }
            }
        }   
        
        while !deq.is_empty() {
            let (i, j, depth) = deq.pop_front().unwrap();

            for (x, y) in directions(i, j, &mat) {
                if !visited.contains(&(x as i32, y as i32)) {
                    visited.insert((x as i32, y as i32));                    
                    deq.push_back((x as i32, y as i32, depth + 1));  
                    ans[x as usize][y as usize] = depth;
                }
            }
        }
        ans 
    }

    /// ## 1293. Shortest Path in a Grid with Obstacles Elimination
    /// https://leetcode.com/problems/shortest-path-in-a-grid-with-obstacles-elimination/
    ///
    /// You are given an m x n integer matrix grid where each cell is either 0 (empty) or 1 (obstacle). 
    /// You can move up, down, left, or right from and to an empty cell in one step.
    ///
    /// Return the minimum number of steps to walk from the upper left corner (0, 0) to the lower right 
    /// corner (m - 1, n - 1) given that you can eliminate at most k obstacles. If it is not possible to 
    /// find such walk return -1.
    ///
    /// Example 1:
    /// -----------
    /// Input: grid = [[0,0,0],[1,1,0],[0,0,0],[0,1,1],[0,0,0]], k = 1
    ///
    /// Output: 6
    /// 
    /// Explanation: 
    /// The shortest path without eliminating any obstacle is 10.
    /// The shortest path with one obstacle elimination at position (3,2) is 6. 
    /// Such path is (0,0) -> (0,1) -> (0,2) -> (1,2) -> (2,2) -> (3,2) -> (4,2).
    /// 
    /// Example 2:
    /// -----------
    /// Input: grid = [[0,1,1],[1,1,1],[1,0,0]], k = 1
    ///
    /// Output: -1
    /// 
    /// Explanation: We need to eliminate at least two obstacles to find such a walk.
    ///
    /// Constraints:
    /// -----------
    /// m == grid.length
    /// n == grid[i].length
    /// 1 <= m, n <= 40
    /// 1 <= k <= m * n
    /// grid[i][j] is either 0 or 1.
    /// grid[0][0] == grid[m - 1][n - 1] == 0
    ///
    /// TODO: Fix test
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        fn directions(grid: &Vec<Vec<i32>>, i: i32, j: i32) -> Vec<(i32, i32)> {
            let mut ans: Vec<(i32, i32)> = Vec::new();
            let points: Vec<(i32, i32)> = vec![(0, -1), (-1, 0), (0, 1), (1, 0)];
            for (x, y) in points {
                if (i + x >= 0 && j + y >= 0) &&
                (i + x < grid.len() as i32 && j + y < grid[0].len() as i32) {
                    ans.push((i + x, j + y));
                }
            }
            ans
        }
        
        let mut k: i32 = k;

        // BFS to grid position with k obstacles
        let mut deq: VecDeque<(i32, i32, i32)> = VecDeque::new();
        deq.push_back((0, 0, 0));
        let (m, n) = (grid.len() as i32, grid[0].len() as i32);

        // Save the state in visited with position and k 
        let mut visited: HashSet<(i32, i32, i32)> = HashSet::new();
        visited.insert((0, 0, k));

        while !deq.is_empty() {
            let (i, j, depth) = deq.pop_front().unwrap();
            // If the last row and col reached
            if i == m - 1 && j == n - 1 {
                return depth;
            }

            for (r, c) in directions(&grid, i, j) {
                // This will prevent us from visiting the obstacle that's already visited 
                if visited.contains(&(r, c, k)) {
                    continue;
                }

                // If the obstacle count not reached
                if grid[r as usize][c as usize] == 1 && k > 1 {
                    k -= 1;
                }
                
                deq.push_back((r, c, depth + 1));
                visited.insert((r, c, k));
            }
        }
        -1
    }

    /// ## 1129. Shortest Path with Alternating Colors
    /// https://leetcode.com/problems/shortest-path-with-alternating-colors/
    /// 
    /// You are given an integer n, the number of nodes in a directed graph where the nodes are 
    /// labeled from 0 to n - 1. Each edge is red or blue in this graph, and there could be self-edges 
    /// and parallel edges.
    ///
    /// You are given two arrays redEdges and blueEdges where:
    ///
    /// redEdges[i] = [ai, bi] indicates that there is a directed red edge from node ai to node bi 
    /// in the graph, and blueEdges[j] = [uj, vj] indicates that there is a directed blue edge from 
    /// node uj to node vj in the graph.
    /// Return an array answer of length n, where each answer[x] is the length of the shortest path 
    /// from node 0 to node x such that the edge colors alternate along the path, or -1 if such a 
    /// path does not exist.
    ///
    ///
    /// Example 1:
    /// ----------
    /// Input: n = 3, redEdges = [[0,1],[1,2]], blueEdges = []
    /// Output: [0,1,-1]
    /// 
    /// Example 2:
    /// ----------
    /// Input: n = 3, redEdges = [[0,1]], blueEdges = [[2,1]]
    /// Output: [0,1,-1]
    ///
    ///
    /// Constraints:
    /// -----------
    /// 1 <= n <= 100
    /// 0 <= redEdges.length, blueEdges.length <= 400
    /// redEdges[i].length == blueEdges[j].length == 2
    /// 0 <= ai, bi, uj, vj < n
    /// 
    pub fn shortest_alternating_paths(
        n: i32, red_edges: Vec<Vec<i32>>, blue_edges: Vec<Vec<i32>>
    ) -> Vec<i32> {
        type Graph = HashMap<i32, Vec<i32>>;
        trait GraphT {
            fn new() -> Graph {
                HashMap::new()
            }
        };
        impl GraphT for Graph {}

        // state info - A deque to store nodeid, depth, color
        let mut deq: VecDeque<(i32, i32, bool)> = VecDeque::new();
        let mut ans: Vec<i32> = vec![i32::MAX; n as usize];

        const BLUE: bool = true;
        let mut graph: HashMap<bool, Graph> = HashMap::new();
        
        graph.entry(BLUE).or_insert(Graph::new());
        graph.entry(!BLUE).or_insert(Graph::new());

        for e in red_edges {
            let (source, dest) = (e[0], e[1]);
            if let Some(entry) = graph.get_mut(&!BLUE){
                entry.entry(source).or_insert(vec![]).push(dest);
            }
        }

        for e in blue_edges {
            let (source, dest) = (e[0], e[1]);
            if let Some(entry) = graph.get_mut(&BLUE){
                entry.entry(source).or_insert(vec![]).push(dest);
            }
        }
        // Create a visited set of key (nodeid, color) and start iterating from 0, 0
        let mut visited: HashSet<(i32, bool)> = HashSet::new();
        // Add both the color to the visited set
        visited.insert((0, BLUE)); 
        visited.insert((0, !BLUE));

        deq.push_back((0, 0, BLUE));
        deq.push_back((0, 0, !BLUE));
        
        while !deq.is_empty() {
            let (nodeid, depth, color) = deq.pop_front().unwrap();
            let node = graph.get(&color).unwrap();

            ans[nodeid as usize] = depth.min(ans[nodeid as usize]);

            if let Some(neigh) = node.get(&nodeid) {
                for n1 in neigh {
                    if !visited.contains(&(*n1, !color)) {
                        visited.insert((*n1, !color));
                        deq.push_back((*n1, depth + 1, !color));
                    }
                }
            }
        }

        ans.iter_mut().map(|x| if x == &i32::MAX { -1 } else { *x }).collect()
        
    }

    /// ## 841. Nearest Exit from Entrance in Maze
    /// https://leetcode.com/problems/nearest-exit-from-entrance-in-maze/description/
    ///
    /// You are given an m x n matrix maze (0-indexed) with empty cells (represented as '.') and walls 
    /// (represented as '+'). You are also given the entrance of the maze, where entrance = [entrancerow, entrancecol] 
    /// denotes the row and column of the cell you are initially standing at.
    ///
    /// In one step, you can move one cell up, down, left, or right. You cannot step into a cell with 
    /// a wall, and you cannot step outside the maze. Your goal is to find the nearest exit from the 
    /// entrance. An exit is defined as an empty cell that is at the border of the maze. The entrance 
    /// does not count as an exit.
    ///
    /// Return the number of steps in the shortest path from the entrance to the nearest exit, or -1 if 
    /// no such path exists.
    ///
    /// Example 1:
    /// ----------
    ///
    /// Input: maze = [["+","+",".","+"],[".",".",".","+"],["+","+","+","."]], entrance = [1,2]
    /// 
    /// Output: 1
    /// 
    /// Explanation: There are 3 exits in this maze at [1,0], [0,2], and [2,3].
    /// Initially, you are at the entrance cell [1,2].
    /// - You can reach [1,0] by moving 2 steps left.
    /// - You can reach [0,2] by moving 1 step up.
    /// It is impossible to reach [2,3] from the entrance.
    /// Thus, the nearest exit is [0,2], which is 1 step away.
    ///
    /// Example 2:
    /// ----------
    ///
    /// Input: maze = [["+","+","+"],[".",".","."],["+","+","+"]], entrance = [1,0]
    /// 
    /// Output: 2
    /// 
    /// Explanation: There is 1 exit in this maze at [1,2].
    /// [1,0] does not count as an exit since it is the entrance cell.
    /// Initially, you are at the entrance cell [1,0].
    /// - You can reach [1,2] by moving 2 steps right.
    /// Thus, the nearest exit is [1,2], which is 2 steps away.
    ///
    /// Example 3:
    /// ----------
    ///
    /// Input: maze = [[".","+"]], entrance = [0,0]
    /// 
    /// Output: -1
    /// 
    /// Explanation: There are no exits in this maze.
    ///
    /// Constraints:
    /// -----------
    /// maze.length == m
    /// maze[i].length == n
    /// 1 <= m, n <= 100
    /// maze[i][j] is either '.' or '+'.
    /// entrance.length == 2
    /// 0 <= entrancerow < m
    /// 0 <= entrancecol < n
    /// entrance will always be an empty cell.
    ///
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {

        fn directions(i: i32, j: i32, maze: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
            let mut ans: Vec<(usize, usize)> = vec![];
            let points: Vec<(i32, i32)> = vec![(0, -1), (-1, 0), (0, 1), (1, 0)];
            for (x, y) in points {
                
                if (i + x >= 0 && j + y >= 0) && 
                (i + x < maze.len() as i32 && j + y < maze[0].len() as i32) && 
                maze[(i + x) as usize][(j + y) as usize] == '.' {
                    ans.push(((i + x) as usize, (j + y) as usize));    
                }
                
            }
            ans
        }

        // Deque to store the pos and depth
        let mut deq: VecDeque<(i32, i32, i32)> = VecDeque::new();
        deq.push_back((entrance[0], entrance[1], 0));
        let mut maze = maze.clone();
        maze[entrance[0] as usize][entrance[1] as usize] = '+';

        let (n_rows, n_cols) = (maze.len(), maze[0].len());

        while !deq.is_empty() {
            let (row, col, depth) = deq.pop_front().unwrap();

            for (x, y) in directions(row, col, &maze) {
                if (x == n_rows - 1 || x == 0) || (y == n_cols - 1 || y == 0) {
                    return depth + 1;
                }
                maze[x as usize][y as usize] = '+';
                deq.push_back((x as i32, y as i32, depth + 1));
            }
        }
        -1   
    }
    
    /// ## 909. Snakes and Ladders
    ///
    /// https://leetcode.com/problems/snakes-and-ladders/description/
    ///
    /// You are given an n x n integer matrix board where the cells are labeled from 1 to n2 in a Boustrophedon 
    /// style starting from the bottom left of the board (i.e. board[n - 1][0]) and alternating direction each row.
    ///
    /// You start on square 1 of the board. In each move, starting from square curr, do the following:
    ///
    /// - Choose a destination square next with a label in the range [curr + 1, min(curr + 6, n2)].
    /// - This choice simulates the result of a standard 6-sided die roll: i.e., there are always at most 
    /// 6 destinations, regardless of the size of the board.
    /// - If next has a snake or ladder, you must move to the destination of that snake or ladder. Otherwise, you 
    // move to next.
    /// - The game ends when you reach the square n2.
    ///
    /// A board square on row r and column c has a snake or ladder if board[r][c] != -1. The destination of that 
    /// snake or ladder is board[r][c]. Squares 1 and n2 do not have a snake or ladder.
    ///
    /// Note that you only take a snake or ladder at most once per move. If the destination to a snake or ladder 
    /// is the start of another snake or ladder, you do not follow the subsequent snake or ladder.
    ///
    /// - For example, suppose the board is [[-1,4],[-1,3]], and on the first move, your destination square is 2. 
    /// You follow the ladder to square 3, but do not follow the subsequent ladder to 4.
    ///
    /// Return the least number of moves required to reach the square n2. If it is not possible to reach the square, 
    /// return -1.
    ///
    ///
    /// Example 1:
    /// ----------
    /// Input: board = 
    /// [[-1,-1,-1,-1,-1,-1],
    /// [-1,-1,-1,-1,-1,-1],
    /// [-1,-1,-1,-1,-1,-1],
    /// [-1,35,-1,-1,13,-1],
    /// [-1,-1,-1,-1,-1,-1],
    /// [-1,15,-1,-1,-1,-1]]
    /// 
    /// Output: 4
    /// 
    /// Explanation: 
    /// In the beginning, you start at square 1 (at row 5, column 0).
    /// You decide to move to square 2 and must take the ladder to square 15.
    /// You then decide to move to square 17 and must take the snake to square 13.
    /// You then decide to move to square 14 and must take the ladder to square 35.
    /// You then decide to move to square 36, ending the game.
    /// This is the lowest possible number of moves to reach the last square, so return 4.
    ///
    /// Example 2:
    /// ----------
    /// Input: board = [[-1,-1],[-1,3]]
    /// 
    /// Output: 1
    ///
    /// Constraints:
    /// ----------
    /// n == board.length == board[i].length
    /// 2 <= n <= 20
    /// board[i][j] is either -1 or in the range [1, n2].
    /// The squares labeled 1 and n2 do not have any ladders or snakes.
    ///
    /// TODO: Fix test cases
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        fn directions(row: i32, col: i32, board: &Vec<Vec<i32>>) -> Vec<(i32, i32)> {
            // This will return the all possible node that can be visited
            // for a die roll
            let dim: i32 = board.len() as i32;
            let curr: i32 = (row * dim) + col;
            let mut ans: Vec<(i32, i32)> = Vec::new();

            for i in curr+1..std::cmp::min(curr + 7, dim.pow(2)) {
                // This will give the new coordinates
                let (x, y) = (i / dim, i % dim);
                if board[x as usize][y as usize] != -1 {
                    let new_pos = board[x as usize][y as usize] - 1;
                    ans.push((new_pos / dim, new_pos % dim));
                } else {
                    ans.push((x, y));
                }
            }
            
            ans
        }
        
        let dim: i32 = board.len() as i32;
        // deque with position and depth
        let mut deq: VecDeque<(i32, i32, i32)> = VecDeque::new();
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        
        // Add coordinate to visited
        visited.insert((0, 0));
        deq.push_back((0, 0, 0));
        
        // Reverse the row to start from 0 index
        let mut board = board.clone();
        board.reverse();
        
        while !deq.is_empty() {
            let (row, col, depth) = deq.pop_front().unwrap();

            if row == dim - 1 && col == dim -1 {
                return depth;
            }

            for (r, c) in directions(row, col, &board) {
                if !visited.contains(&(r, c)) {
                    visited.insert((r, c));
                    deq.push_back((r, c, depth + 1));
                }
            }
        }
        -1
    }

    /// ## 752. Open the Lock
    /// https://leetcode.com/problems/open-the-lock/
    /// 
    /// You have a lock in front of you with 4 circular wheels. Each wheel has 10 slots: 
    /// ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']. The wheels can rotate freely and wrap around: 
    /// for example we can turn '9' to be '0', or '0' to be '9'. Each move consists of turning one wheel one slot.
    ///
    /// The lock initially starts at '0000', a string representing the state of the 4 wheels.
    ///
    /// You are given a list of deadends dead ends, meaning if the lock displays any of these codes, the wheels 
    /// of the lock will stop turning and you will be unable to open it.
    ///
    /// Given a target representing the value of the wheels that will unlock the lock, return the minimum total 
    /// number of turns required to open the lock, or -1 if it is impossible.
    ///
    /// Example 1:
    /// ----------
    /// Input: deadends = ["0201","0101","0102","1212","2002"], target = "0202"
    /// Output: 6
    /// Explanation: 
    /// A sequence of valid moves would be "0000" -> "1000" -> "1100" -> "1200" -> "1201" -> "1202" -> "0202".
    /// Note that a sequence like "0000" -> "0001" -> "0002" -> "0102" -> "0202" would be invalid,
    /// because the wheels of the lock become stuck after the display becomes the dead end "0102".
    ///
    /// Example 2:
    /// ----------
    /// Input: deadends = ["8888"], target = "0009"
    /// Output: 1
    /// Explanation: We can turn the last wheel in reverse to move from "0000" -> "0009".
    /// 
    /// Example 3:
    /// ----------
    /// Input: deadends = ["8887","8889","8878","8898","8788","8988","7888","9888"], target = "8888"
    /// Output: -1
    /// Explanation: We cannot reach the target without getting stuck.
    ///
    /// Constraints:
    /// -----------
    /// 1 <= deadends.length <= 500
    /// deadends[i].length == 4
    /// target.length == 4
    /// target will not be in the list deadends.
    /// target and deadends[i] consist of digits only.
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        // Function to generate all possible paths from the current position
        // Visited sets to keep track of already visited positions
        // Deque to store all possible paths from current position and depth of current path

        fn generate_paths(start: Vec<char>) -> Vec<Vec<char>> {
            let mut ans: Vec<Vec<char>> = Vec::new();

            for i in 0..start.len() {
                let curr_digit = start[i].to_digit(10).unwrap() as i32;
                
                for j in vec![-1, 1] {
                    let mut new_start: Vec<char> = start.clone();
                    let mut new_digit = ((curr_digit + j) % 10);
                    if new_digit < 0 { new_digit = 9; }
                    
                    if let Some(x) = new_digit.to_string().chars().nth(0) {
                        new_start[i as usize] = x;
                        ans.push(new_start);
                    }
                }   
            }
            ans
        }
        // If target is in deadend
        for deadend in deadends.iter() {
            if *deadend == "0000".to_string() { return -1; }
        }

        let start: Vec<char> = "0000".chars().into_iter().collect();
        let target: Vec<char> = target.chars().into_iter().collect();

        // This will hold current node and depth of current path
        let mut deq: VecDeque<(Vec<char>, i32)> = VecDeque::new();
        deq.push_back((start.clone(), 0));
        
        let mut visited: HashSet<Vec<char>> = HashSet::new();
        visited.insert(start.clone());
        visited.extend(deadends.iter().map(|x| x.chars().into_iter().collect()));

        while let Some((path, depth)) = deq.pop_front() {
            if path == target {
                return depth;
            }
            for child in generate_paths(path) {
                if !visited.contains(&child) {
                    deq.push_back((child.clone(), depth + 1));
                    visited.insert(child.clone());
                }
            }
        }
        -1
    }

    /// ## 399. Evaluate Division
    /// https://leetcode.com/problems/evaluate-division/
    ///
    /// You are given an array of variable pairs equations and an array of real numbers values, where 
    /// equations[i] = [Ai, Bi] and values[i] represent the equation Ai / Bi = values[i]. Each Ai or Bi 
    /// is a string that represents a single variable.
    ///
    /// You are also given some queries, where queries[j] = [Cj, Dj] represents the jth query where you 
    /// must find the answer for Cj / Dj = ?.
    ///
    /// Return the answers to all queries. If a single answer cannot be determined, return -1.0.
    ///
    /// Note: The input is always valid. You may assume that evaluating the queries will not result in 
    /// division by zero and that there is no contradiction.
    ///
    /// Example 1:
    /// ----------
    /// Input: equations = [["a","b"],["b","c"]], values = [2.0,3.0], queries = [["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]]
    ///
    /// Output: [6.00000,0.50000,-1.00000,1.00000,-1.00000]
    /// Explanation: 
    /// Given: a / b = 2.0, b / c = 3.0
    /// queries are: a / c = ?, b / a = ?, a / e = ?, a / a = ?, x / x = ?
    /// return: [6.0, 0.5, -1.0, 1.0, -1.0 ]
    ///
    /// Example 2:
    /// ----------
    /// Input: equations = [["a","b"],["b","c"],["bc","cd"]], values = [1.5,2.5,5.0], queries = [["a","c"],["c","b"],["bc","cd"],["cd","bc"]]
    /// Output: [3.75000,0.40000,5.00000,0.20000]
    ///
    /// Example 3:
    /// ----------
    /// Input: equations = [["a","b"]], values = [0.5], queries = [["a","b"],["b","a"],["a","c"],["x","y"]]
    ///
    /// Output: [0.50000,2.00000,-1.00000,-1.00000]
    ///
    /// Constraints:
    /// ------------
    /// 1 <= equations.length <= 20
    /// equations[i].length == 2
    /// 1 <= Ai.length, Bi.length <= 5
    /// values.length == equations.length
    /// 0.0 < values[i] <= 20.0
    /// 1 <= queries.length <= 20
    /// queries[i].length == 2
    /// 1 <= Cj.length, Dj.length <= 5
    /// Ai, Bi, Cj, Dj consist of lower case English letters and digits.
    /// 
    pub fn calc_equation(
        equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>
    ) -> Vec<f64> {
        let mut ans: Vec<f64> = vec![-1.0; queries.len()];
        
        // Build graph
        let mut graph: HashMap<String, Vec<(String, f64)>> = HashMap::new();
        for i in 0..equations.len() {
            let entry: &mut Vec<(String, f64)> = 
                graph.entry(equations[i][0].clone())
                .or_insert(Vec::new());
            entry.push((equations[i][1].clone(), values[i]));

            let entry: &mut Vec<(String, f64)> = 
                graph.entry(equations[i][1].clone())
                .or_insert(Vec::new());
            entry.push((equations[i][0].clone(), 1.0 / values[i]));
        }

        fn bfs(
            graph: &HashMap<String, Vec<(String, f64)>>, 
            start: &String, end: &String
        ) -> f64 {
            let mut deq: VecDeque<(String, f64)> = VecDeque::new();
            let mut visited: HashSet<String> = HashSet::new();
            deq.push_back((start.clone(), 1.0));
            visited.insert(start.clone());

            while !deq.is_empty() {
                let (cur, prod) = deq.pop_front().unwrap();
                if cur == end.clone() {
                    return prod;
                }

                if let Some(children) = graph.get(&cur) {
                    for child in children {
                        if !visited.contains(&child.0) {
                            visited.insert(child.0.clone());
                            deq.push_back((child.0.clone(), prod * child.1));
                        }
                    }
                }
            }    
        
            -1.0
        }

        // Process query
        for i in 0..queries.len() {
            
            if !graph.contains_key(&queries[i][0]) || !graph.contains_key(&queries[i][1]) {
                ans[i] = -1.0;    
            } else if queries[i][0] == queries[i][1] {
                ans[i] = 1.0;
            } else {
                ans[i] = bfs(&graph, &queries[i][0], &queries[i][1]);
            }
        }

        ans
    }

    /// ## Minimum Genetic Mutation
    /// https://leetcode.com/problems/minimum-genetic-mutation/description/
    ///
    /// A gene string can be represented by an 8-character long string, with choices from 'A', 'C', 'G', and 'T'.
    ///
    /// Suppose we need to investigate a mutation from a gene string startGene to a gene string endGene where one 
    /// mutation is defined as one single character changed in the gene string.
    ///
    /// For example, "AACCGGTT" --> "AACCGGTA" is one mutation.
    /// There is also a gene bank bank that records all the valid gene mutations. A gene must be in bank to make 
    /// it a valid gene string.
    ///
    /// Given the two gene strings startGene and endGene and the gene bank bank, return the minimum number of mutations 
    /// needed to mutate from startGene to endGene. If there is no such a mutation, return -1.
    ///
    /// Note that the starting point is assumed to be valid, so it might not be included in the bank.
    ///
    ///
    /// Example 1:
    /// ----------
    /// Input: startGene = "AACCGGTT", endGene = "AACCGGTA", bank = ["AACCGGTA"]
    /// Output: 1
    ///
    /// Example 2:
    /// ----------
    /// Input: startGene = "AACCGGTT", endGene = "AAACGGTA", bank = ["AACCGGTA","AACCGCTA","AAACGGTA"]
    /// Output: 2
    ///
    /// Constraints:
    /// -----------
    /// 0 <= bank.length <= 10
    /// startGene.length == endGene.length == bank[i].length == 8
    /// startGene, endGene, and bank[i] consist of only the characters ['A', 'C', 'G', 'T'].
    ///
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        fn directions(start: Vec<char>) -> Vec<Vec<char>> {
            let mut ans: Vec<Vec<char>> = Vec::new();
            // Generate all possible directions using 'A', 'C', 'G', and 'T'
            for i in 0..start.len() {
                for j in vec!['A', 'C', 'G', 'T'] {
                    let mut new_start: Vec<char> = start.clone();
                    new_start[i] = j;
                    ans.push(new_start);
                }
            }
            ans
        }
        let end_gene: Vec<char> = end_gene.chars().collect();
        let start_gene: Vec<char> = start_gene.chars().collect();

        let mut deq: VecDeque<(Vec<char>, i32)> = VecDeque::new();
        deq.push_back((start_gene.clone(), 0));

        // This will let us search in O(1) time
        let bank: HashSet<Vec<char>> = bank.iter().map(|s| s.chars().collect()).collect();
        
        let mut visited: HashSet<Vec<char>> = HashSet::new();
        visited.insert(start_gene.clone());
        
        while !deq.is_empty() {
            let (cur, cost) = deq.pop_front().unwrap();
            if cur == end_gene {
                return cost;
            }

            for gene in directions(cur) {
                if visited.contains(&gene) {
                    continue;
                }
                if bank.contains(&gene) {
                    visited.insert(gene.clone());
                    deq.push_back((gene.clone(), cost + 1));
                }
            }
        }
        -1
    }

    /// ## 1557. Minimum Number of Vertices to Reach All Nodes
    ///
    /// Given a directed acyclic graph, with n vertices numbered from 0 to n-1, and an array edges where 
    /// edges[i] = [fromi, toi] represents a directed edge from node fromi to node toi.
    ///
    /// Find the smallest set of vertices from which all nodes in the graph are reachable. It's guaranteed that 
    /// a unique solution exists.
    ///
    /// Notice that you can return the vertices in any order.
    ///
    ///
    /// Example 1:
    /// ------------
    /// Input: n = 6, edges = [[0,1],[0,2],[2,5],[3,4],[4,2]]
    ///
    /// Output: [0,3]
    ///
    /// Explanation: It's not possible to reach all the nodes from a single vertex. From 0 we can reach [0,1,2,5]. 
    /// From 3 we can reach [3,4,2,5]. So we output [0,3].
    ///
    /// Example 2:
    /// ------------
    // Input: n = 5, edges = [[0,1],[2,1],[3,1],[1,4],[2,4]]
    ///
    /// Output: [0,2,3]
    ///
    /// Explanation: Notice that vertices 0, 3 and 2 are not reachable from any other node, so we must include them. 
    /// Also any of these vertices can reach nodes 1 and 4.
    ///
    /// Constraints:
    /// -----------
    /// 2 <= n <= 10^5
    /// 1 <= edges.length <= min(10^5, n * (n - 1) / 2)
    /// edges[i].length == 2
    /// 0 <= fromi, toi < n
    /// All pairs (fromi, toi) are distinct.
    ///
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let indegree: HashSet<i32> = edges.iter().map(|edge| edge[1]).collect();
        let total: HashSet<i32> = (0..n).into_iter().map(|vertex| vertex).collect();
        let mut diff: Vec<i32> = Vec::new();
        for i in total.difference(&indegree) {
            diff.push(*i);
        }
        diff
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

    /// ## Detonate the Maximum Bombs
    ///
    /// You are given a list of bombs. The range of a bomb is defined as the area where its effect can be felt. 
    /// This area is in the shape of a circle with the center as the location of the bomb.
    ///
    /// The bombs are represented by a 0-indexed 2D integer array bombs where bombs[i] = [xi, yi, ri]. xi and 
    /// yi denote the X-coordinate and Y-coordinate of the location of the ith bomb, whereas ri denotes the radius 
    /// of its range.
    ///
    /// You may choose to detonate a single bomb. When a bomb is detonated, it will detonate all bombs that lie 
    /// in its range. These bombs will further detonate the bombs that lie in their ranges.
    ///
    /// Given the list of bombs, return the maximum number of bombs that can be detonated if you are allowed to 
    /// detonate only one bomb.
    ///
    /// Example 1:
    /// ---------
    /// Input: bombs = [[2,1,3],[6,1,4]]
    /// 
    /// Output: 2
    /// 
    /// Explanation:
    /// The above figure shows the positions and ranges of the 2 bombs.
    /// If we detonate the left bomb, the right bomb will not be affected.
    /// But if we detonate the right bomb, both bombs will be detonated.
    /// So the maximum bombs that can be detonated is max(1, 2) = 2.
    /// 
    /// Example 2:
    /// ----------
    /// Input: bombs = [[1,1,5],[10,10,5]]
    ///
    /// Output: 1
    /// 
    /// Explanation:
    /// Detonating either bomb will not detonate the other bomb, so the maximum number of bombs that can be detonated is 1.
    /// 
    /// Example 3:
    /// ----------
    /// Input: bombs = [[1,2,3],[2,3,1],[3,4,2],[4,5,3],[5,6,4]]
    ///
    /// Output: 5
    /// 
    /// Explanation:
    /// The best bomb to detonate is bomb 0 because:
    /// - Bomb 0 detonates bombs 1 and 2. The red circle denotes the range of bomb 0.
    /// - Bomb 2 detonates bomb 3. The blue circle denotes the range of bomb 2.
    /// - Bomb 3 detonates bomb 4. The green circle denotes the range of bomb 3.
    /// Thus all 5 bombs are detonated.
    ///
    /// Constraints:
    /// -----------
    /// 1 <= bombs.length <= 100
    /// bombs[i].length == 3
    /// 1 <= xi, yi, ri <= 105
    ///
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        unimplemented!()   
    }

    /// ## Word Ladder
    ///
    /// A transformation sequence from word beginWord to word endWord using a dictionary wordList is a sequence of 
    /// words beginWord -> s1 -> s2 -> ... -> sk such that:
    ///
    /// Every adjacent pair of words differs by a single letter.
    /// Every si for 1 <= i <= k is in wordList. Note that beginWord does not need to be in wordList.
    /// sk == endWord
    /// Given two words, beginWord and endWord, and a dictionary wordList, return the number of words in the shortest 
    /// transformation sequence from beginWord to endWord, or 0 if no such sequence exists.
    ///
    /// Example 1:
    /// ----------
    /// Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
    /// 
    /// Output: 5
    /// 
    /// Explanation: One shortest transformation sequence is "hit" -> "hot" -> "dot" -> "dog" -> cog", which is 5 words 
    /// long.
    ///
    /// Example 2:
    /// ----------
    /// Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
    /// 
    /// Output: 0
    /// 
    /// Explanation: The endWord "cog" is not in wordList, therefore there is no valid transformation sequence.
    ///
    /// Constraints:
    /// -----------
    /// 1 <= beginWord.length <= 10
    /// endWord.length == beginWord.length
    /// 1 <= wordList.length <= 5000
    /// wordList[i].length == beginWord.length
    /// beginWord, endWord, and wordList[i] consist of lowercase English letters.
    /// beginWord != endWord
    /// All the words in wordList are unique.
    ///
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        unimplemented!()    
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_str_to_string(str: Vec<&str>) -> Vec<String> {
        let mut str = str.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        str
    }

    #[test]
    fn test_find_circle_num() {
        let is_connected = vec![vec![1,1,0],vec![1,1,0],vec![0,0,1]];
        assert_eq!(Solution::find_circle_num(is_connected), 2);

        let is_connected = vec![vec![1,0,0], vec![0,1,0], vec![0,0,1]];
        assert_eq!(Solution::find_circle_num(is_connected), 3);
    }

    #[test]
    fn test_min_reorder() {
        let n = 6; 
        let connections = vec![vec![0,1],vec![1,3],vec![2,3],vec![4,0],vec![4,5]];
        assert_eq!(Solution::min_reorder(n, connections), 3);

        let n = 5; 
        let connections = vec![vec![1,0],vec![1,2],vec![3,2],vec![3,4]];
        assert_eq!(Solution::min_reorder(n, connections), 2);

        let n = 3; 
        let connections = vec![vec![1,0],vec![2,0]];
        assert_eq!(Solution::min_reorder(n, connections), 0);
    }

    #[test]
    fn test_can_visit_all_rooms() {
        let rooms = vec![vec![1],vec![2],vec![3],vec![]]; 
        assert_eq!(Solution::can_visit_all_rooms(rooms), true);

        let rooms: Vec<Vec<i32>> = vec![vec![1,3],vec![3,0,1],vec![2],vec![0]];
        assert_eq!(Solution::can_visit_all_rooms(rooms), false);
    }

    #[test]
    fn test_shortest_path_binary_matrix() {
        let grid = vec![vec![0,1],vec![1,0]];
        assert_eq!(Solution::shortest_path_binary_matrix(grid), 2);
    
        
        let grid = vec![vec![0,0,0],vec![1,1,0],vec![1,1,0]];
        assert_eq!(Solution::shortest_path_binary_matrix(grid), 4);
        
        let grid = vec![vec![1,0,0], vec![1,1,0], vec![1,1,0]];
        assert_eq!(Solution::shortest_path_binary_matrix(grid), -1);
    }

    #[test]
    fn test_shortest_path() {
        let grid: Vec<Vec<i32>> = vec![
            vec![0,0,0],vec![1,1,0],vec![0,0,0],vec![0,1,1],vec![0,0,0]
        ]; 
        let k = 1;
        assert_eq!(Solution::shortest_path(grid, k), 6);

        let grid: Vec<Vec<i32>> = vec![
            vec![0,1,1],vec![1,1,1],vec![1,0,0]]; 
        let k = 1;
        assert_eq!(Solution::shortest_path(grid, k), -1);
    }

    #[test]
    fn test_update_matrix() {
        let mat = vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]];
        let ans = vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]];
        assert_eq!(Solution::update_matrix(mat), ans);
        
        let mat = vec![vec![0,0,0],vec![0,1,0],vec![1,1,1]];
        let ans = vec![vec![0,0,0],vec![0,1,0],vec![1,2,1]];
        assert_eq!(Solution::update_matrix(mat), ans);
    }

    #[test]
    fn test_shortest_alternating_paths() {
        
        let n = 3;
        let red_edges = vec![vec![0,1],vec![1,2]];
        let blue_edges: Vec<Vec<i32>> = vec![];
        let ans = vec![0,1,-1];
        assert_eq!(Solution::shortest_alternating_paths(n, red_edges, blue_edges), ans);
        
        let n = 3;
        let red_edges = vec![vec![0,1]];
        let blue_edges = vec![vec![2,1]];
        let ans = vec![0,1,-1];
        assert_eq!(Solution::shortest_alternating_paths(n, red_edges, blue_edges), ans);
    }

    #[test]
    fn test_nearest_exit() {
        let maze = vec![vec!['+','+','.','+'],vec!['.','.','.','+'],vec!['+','+','+','.']];
        let entrance = vec![1,2];
        assert_eq!(Solution::nearest_exit(maze, entrance), 1);
        
        let maze = vec![vec!['+','+','+'], vec!['.','.','.'], vec!['+','+','+']];
        let entrance = vec![1,0];
        assert_eq!(Solution::nearest_exit(maze, entrance), 2);
        
        let maze = vec![vec!['.','+']]; let entrance = vec![0,0];
        assert_eq!(Solution::nearest_exit(maze, entrance), -1);

        let maze = vec![
            vec!['+','.','+','+','+','+','+'],
            vec!['+','.','+','.','.','.','+'],
            vec!['+','.','+','.','+','.','+'],
            vec!['+','.','.','.','+','.','+'],
            vec!['+','+','+','+','+','.','+']
        ];
        let entrance = vec![0,1];
        assert_eq!(Solution::nearest_exit(maze, entrance), 12);
    }

    #[test]
    fn test_cal_equation() {
        let vecstr_to_vecstring: fn(v: Vec<&str>) -> Vec<String> = 
            |v| v.iter().map(|x| x.to_string()).collect();
        
        // Test case 1
        let equations: Vec<Vec<String>>  = 
            vec![vec!["a","b"], vec!["b","c"]]
            .iter().map(|v| vecstr_to_vecstring(v.clone())).collect(); 
        let values = vec![2.0,3.0]; 
        let queries: Vec<Vec<String>>  = 
            vec![(vec!["a","c"]),vec!["b","a"], vec!["a","e"], vec!["a","a"], vec!["x","x"]]
            .iter().map(|v| vecstr_to_vecstring(v.clone())).collect();

        let ans = vec![6.00000,0.50000,-1.00000,1.00000,-1.00000];
        assert_eq!(Solution::calc_equation(equations, values, queries), ans);

        // Test case 2
        let equations: Vec<Vec<String>> = vec![vecstr_to_vecstring(vec!["a","b"])];
        let values: Vec<f64> = vec![0.5]; 
        let queries: Vec<Vec<String>> = 
            vec![vec!["a","b"], vec!["b","a"], vec!["a","c"], vec!["x","y"]]
            .iter().map(|v| vecstr_to_vecstring(v.clone())).collect();
        let ans = vec![0.50000,2.00000,-1.00000,-1.00000];
        assert_eq!(Solution::calc_equation(equations, values, queries), ans);
        
        // Test case 3
        let equations = 
            vec![vec!["a","b"], vec!["b","c"], vec!["bc","cd"]]
            .iter().map(|v| vecstr_to_vecstring(v.clone())).collect();
        let values = vec![1.5,2.5,5.0]; 
        let queries = 
            vec![ vec!["a","c"], vec!["c","b"], vec!["bc","cd"], vec!["cd","bc"]]
            .iter().map(|v| vecstr_to_vecstring(v.clone())).collect();

        let ans = vec![3.75000,0.40000,5.00000,0.20000];
        assert_eq!(Solution::calc_equation(equations, values, queries), ans);
    }

    #[test]
    fn test_snakes_and_ladders() {
        let board: Vec<Vec<i32>> = vec![
            vec![-1,-1,-1,-1,-1,-1],
            vec![-1,-1,-1,-1,-1,-1],
            vec![-1,-1,-1,-1,-1,-1],
            vec![-1,35,-1,-1,13,-1],
            vec![-1,-1,-1,-1,-1,-1],
            vec![-1,15,-1,-1,-1,-1]];
        assert_eq!(Solution::snakes_and_ladders(board), 4);
    
        let board = vec![vec![-1,-1],vec![-1,3]];
        assert_eq!(Solution::snakes_and_ladders(board), 1);

        let board = vec![
            vec![-1,-1,19,10,-1],
            vec![2,-1,-1,6,-1],
            vec![-1,17,-1,19,-1],
            vec![25,-1,20,-1,-1],
            vec![-1,-1,-1,-1,15]];
        assert_eq!(Solution::snakes_and_ladders(board), 2);
    }
    
    #[test]
    fn test_open_lock() {
        let deadends = vec!["8888".to_string()]; 
        let target = "0009".to_string();
        assert_eq!(Solution::open_lock(deadends, target), 1);

        let deadends: Vec<String>  = 
            vec_str_to_string(vec!["0201","0101","0102","1212","2002"]); 
        let target = "0202".to_string();
        assert_eq!(Solution::open_lock(deadends, target), 6);
        
        let deadends = 
            vec_str_to_string(vec!["8887","8889","8878","8898","8788","8988","7888","9888"]);
        let target = "8888".to_string();
        assert_eq!(Solution::open_lock(deadends, target), -1);
    }

    #[test]
    fn test_min_mutation() {
        let start_gene = "AACCGGTT".to_string(); let end_gene = "AACCGGTA".to_string(); 
        let bank = vec!["AACCGGTA".to_string()];
        assert_eq!(Solution::min_mutation(start_gene, end_gene, bank), 1);
        
        let start_gene = "AACCGGTT".to_string(); 
        let end_gene = "AAACGGTA".to_string(); 
        let bank = vec_str_to_string(vec!["AACCGGTA","AACCGCTA","AAACGGTA"]);
        assert_eq!(Solution::min_mutation(start_gene, end_gene, bank), 2);
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

    #[test]
    fn test_maximum_detonation() {
        let bombs = vec![vec![2,1,3],vec![6,1,4]];
        assert_eq!(Solution::maximum_detonation(bombs), 2);
        let bombs = vec![vec![1,1,5],vec![10,10,5]];
        assert_eq!(Solution::maximum_detonation(bombs), 1);
        let bombs = vec![vec![1,2,3],vec![2,3,1],vec![3,4,2],vec![4,5,3],vec![5,6,4]];
        assert_eq!(Solution::maximum_detonation(bombs), 5);
    }

    #[test]
    fn test_ladder_length() {
        let begin_word = "hit".to_string(); let end_word = "cog".to_string(); 
        let word_list = vec_str_to_string(vec!["hot","dot","dog","lot","log","cog"]);
        assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 5);

        let begin_word = "hit".to_string(); let end_word = "cog".to_string();
        let word_list = vec_str_to_string(vec!["hot","dot","dog","lot","log"]);
        assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 0); 
        
    }


}
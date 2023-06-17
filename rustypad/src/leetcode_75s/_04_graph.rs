#![allow(unused)]
use std::collections::HashSet;

struct Solution;

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
    /// - Input: numCourses = 2, prerequisites = [[1,0]]
    /// - Output: true
    /// - Explanation: There are a total of 2 courses to take. 
    /// To take course 1 you should have finished course 0. So it is possible.
    /// 
    /// Example 2:
    /// ----------
    /// - Input: numCourses = 2, prerequisites = [[1,0],[0,1]]
    /// - Output: false
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
        unimplemented!()
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
}
#![allow(unused)]
use std::collections::{VecDeque, HashSet};

struct Solution;

impl Solution {
    /// ## 490. The Maze
    /// https://leetcode.com/problems/the-maze/description/
    ///
    /// There is a ball in a maze with empty spaces (represented as 0) and walls (represented as 1). 
    /// The ball can go through the empty spaces by rolling up, down, left or right, but it won't stop 
    /// rolling until hitting a wall. When the ball stops, it could choose the next direction.
    ///
    /// Given the m x n maze, the ball's start position and the destination, 
    /// where start = [startrow, startcol] and destination = [destinationrow, destinationcol], return true 
    /// if the ball can stop at the destination, otherwise return false.
    ///
    /// You may assume that the borders of the maze are all walls (see examples).
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let maze = vec![vec![0,0,1,0,0],vec![0,0,0,0,0],vec![0,0,0,1,0],vec![1,1,0,1,1],vec![0,0,0,0,0]]; 
    /// let start = vec![0,4]; let destination = vec![4,4];
    /// assert_eq!(Solution::has_path(maze, start, destination), true);
    /// ```
    /// **Explanation:** One possible way is : left -> down -> left -> down -> right -> down -> right.
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let maze = vec![vec![0,0,1,0,0], vec![0,0,0,0,0],vec![0,0,0,1,0],vec![1,1,0,1,1],vec![0,0,0,0,0]];
    /// let start = vec![0,4]; let destination = vec![3,2];
    /// assert_eq!(Solution::has_path(maze, start, destination), false);
    /// ```
    /// **Explanation:** There is no way for the ball to stop at the destination. Notice that you can pass through the destination 
    /// but you cannot stop there.
    ///
    /// Example 3:
    /// ----------
    /// ```
    /// let maze = vec![vec![0,0,0,0,0],vec![1,1,0,0,1],vec![0,0,0,0,0], vec![0,1,0,0,1],vec![0,1,0,0,0]];
    /// let start = vec![4,3]; let destination = vec![0,1];
    /// assert_eq!(Solution::has_path(maze, start, destination), false);
    /// ```
    ///
    /// Constraints:
    /// ------------
    /// - m == maze.length
    /// - n == maze[i].length
    /// - 1 <= m, n <= 100
    /// - maze[i][j] is 0 or 1.
    /// - start.length == 2
    /// - destination.length == 2
    /// - 0 <= startrow, destinationrow <= m
    /// - 0 <= startcol, destinationcol <= n
    /// - Both the ball and the destination exist in an empty space, and they will not be in the same position initially.
    /// - The maze contains at least 2 empty spaces.
    pub fn has_path(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> bool {
        let m = maze.len();
        let n = maze[0].len();
        let mut visit = vec![vec![false; n]; m];
        let mut queue = VecDeque::new();

        queue.push_back(start.to_owned());
        visit[start[0] as usize][start[1] as usize] = true;
        let dir_x = [0, 1, 0, -1];
        let dir_y = [-1, 0, 1, 0];

        while let Some(curr) = queue.pop_front() {
            if curr[0] == destination[0] && curr[1] == destination[1] {
                return true;
            }

            for i in 0..4 {
                let mut r = curr[0];
                let mut c = curr[1];
                // Move the ball in the chosen direction until it can.
                while r >= 0 && r < m as i32 && c >= 0 && c < n as i32 && maze[r as usize][c as usize] == 0 {
                    r += dir_x[i];
                    c += dir_y[i];
                }
                // Revert the last move to get the cell to which the ball rolls.
                r -= dir_x[i];
                c -= dir_y[i];
                if !visit[r as usize][c as usize] {
                    queue.push_back(vec![r, c]);
                    visit[r as usize][c as usize] = true;
                }
            }
        }

        false
    }
}

mod test {
    use super::*;

    #[test]
    fn test_has_path() {
        let maze = vec![vec![0,0,1,0,0],vec![0,0,0,0,0],vec![0,0,0,1,0],vec![1,1,0,1,1],vec![0,0,0,0,0]];
        let start = vec![0,4]; let destination = vec![4,4];
        assert_eq!(Solution::has_path(maze, start, destination), true);
    }

    #[test]
    fn test_has_path_false() {
        let maze = vec![vec![0,0,1,0,0], vec![0,0,0,0,0],vec![0,0,0,1,0],vec![1,1,0,1,1],vec![0,0,0,0,0]];
        let start = vec![0,4]; let destination = vec![3,2];
        assert_eq!(Solution::has_path(maze, start, destination), false);
    }

    #[test]
    fn test_has_path_false2() {
        let maze = vec![vec![0,0,0,0,0],vec![1,1,0,0,1],vec![0,0,0,0,0], vec![0,1,0,0,1],vec![0,1,0,0,0]];
        let start = vec![4,3]; let destination = vec![0,1];
        assert_eq!(Solution::has_path(maze, start, destination), false);
    }
}
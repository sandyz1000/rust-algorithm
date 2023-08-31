#![allow(unused)]

use std::{collections::{BinaryHeap, HashSet}, cmp::Reverse};

struct Solution;


struct UnionFind {
    parent: Vec<i32>,
    rank: Vec<i32>
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).into_iter().map(|x| x as i32).collect(),
            rank: vec![1; size]
        }
    }

    fn find(&mut self, n1: i32) -> i32 {
        if n1 == self.parent[n1 as usize] {
            return n1;
        }
        // Recursively find the parent of node in the heirarchy chain
        self.parent[n1 as usize] = self.find(self.parent[n1 as usize]);
        self.parent[n1 as usize]
    }

    fn union(&mut self, n1: i32, n2: i32) {
        let parent_1 = self.find(n1);
        let parent_2 = self.find(n2);
        // If the parent not connected; connect the one with higher rank
        if self.rank[parent_1 as usize] > self.rank[parent_2 as usize] {
            self.parent[parent_2 as usize] = parent_1;
        } else if self.rank[parent_1 as usize] < self.rank[parent_2 as usize] {
            self.parent[parent_1 as usize] = parent_2;
        } else {
            // Make either of the node as parent
            self.parent[parent_1 as usize] = parent_2;
            self.rank[parent_2 as usize] += 1;
        }
    }

    fn is_connected(&mut self, n1: i32, n2: i32) -> bool {
        self.find(n1) == self.find(n2)
    }
}

impl Solution {
    /// ## 1584. Min Cost to Connect All Points
    /// https://leetcode.com/problems/min-cost-to-connect-all-points/description/
    ///
    /// You are given an array points representing integer coordinates of some points on 
    /// a 2D-plane, where points[i] = [xi, yi].
    ///
    /// The cost of connecting two points [xi, yi] and [xj, yj] is the manhattan distance 
    /// between them: |xi - xj| + |yi - yj|, where |val| denotes the absolute value of val.
    ///
    /// Return the minimum cost to make all points connected. All points are connected if 
    /// there is exactly one simple path between any two points.
    ///
    /// Example 1:
    /// ---------
    /// ```
    /// let points = vec![vec![0,0],vec![2,2],vec![3,10],vec![5,2],vec![7,0]];
    /// assert_eq!(Solution::min_cost_connect_points(points), 20);
    /// ```
    /// *Explanation*:
    /// We can connect the points as shown above to get the minimum cost of 20.
    /// Notice that there is a unique path between every pair of points.
    ///
    /// Example 2:
    /// ---------
    /// ```
    /// let points = vec![vec![3,12],vec![-2,5],vec![-4,1]];
    /// assert_eq!(Solution::min_cost_connect_points(points), 18);
    /// ```
    /// Constraints:
    /// ------------
    /// - 1 <= points.length <= 1000
    /// - -106 <= xi, yi <= 106
    /// - All pairs (xi, yi) are distinct.
    ///
    pub fn min_cost_connect_points_kruskal(points: Vec<Vec<i32>>) -> i32 {
        // Use kruskal to find min cost to connect all points
        // Time complexity for building bin-heap: O(ElogE)
        // Space Complexity: O(E). We need the space to store all the edges in a priority queue.

        let mut heap: BinaryHeap<(Reverse<i32>, i32, i32)> = BinaryHeap::new();
        let mut union_find: UnionFind = UnionFind::new(points.len());
        for p1 in 0..points.len() {
            for p2 in p1+1..points.len() {
                let x_dist = (points[p2][0] - points[p1][0]).abs();
                let y_dist = (points[p2][1] - points[p1][1]).abs();
                heap.push((Reverse(x_dist+y_dist), p1 as i32, p2 as i32));
            }
        }

        let mut res = 0;
        let mut count = points.len() - 1;  // Max no of connected edge is n-1

        while !heap.is_empty() && count > 0 {
            let (edge, p1, p2) = heap.pop().unwrap();
            if !union_find.is_connected(p1, p2) {
                union_find.union(p1, p2);
                res += edge.0;
                count -= 1;
            }
        }

        res
    }

    pub fn min_cost_connect_points_prim(points: Vec<Vec<i32>>) -> i32 {
        // Prim algorithm similar to kruskal only it expand from the first point
        // Calculate the cost of each edge from the first point to all other points
        // Popped the min edge node from the heap and connect it to all other points
        // 
        let mut heap: BinaryHeap<(Reverse<i32>, i32, i32)> = BinaryHeap::new();
        let mut visited: HashSet<i32> = HashSet::new();
        let mut res = 0;
        let mut count = points.len() - 1;  // Max no of connected edge is n-1

        for p2 in 1..points.len() {
            let x_dist = (points[p2][0] - points[0][0]).abs();
            let y_dist = (points[p2][1] - points[0][1]).abs();
            heap.push((Reverse(x_dist+y_dist), 0, p2 as i32));
        }
        visited.insert(0);
        while !heap.is_empty() && count > 0 {
            let (edge, _, p2) = heap.pop().unwrap();
            if visited.contains(&p2) {
                continue;
            }
            
            res += edge.0;
            // Insert the current node to the visited set
            visited.insert(p2);
            
            for p3 in 0..points.len() {
                if !visited.contains(&(p3 as i32)) {
                    let x_dist = (points[p3][0] - points[p2 as usize][0]).abs();
                    let y_dist = (points[p3][1] - points[p2 as usize][1]).abs();
                    heap.push((Reverse(x_dist+y_dist), p2 as i32, p3 as i32));
                }
            }
            count -= 1;
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let points = vec![vec![0,0],vec![2,2],vec![3,10],vec![5,2],vec![7,0]];
        assert_eq!(Solution::min_cost_connect_points_kruskal(points), 20);
    }

    #[test]
    fn test2() {
        let points = vec![vec![3,12],vec![-2,5],vec![-4,1]];
        assert_eq!(Solution::min_cost_connect_points_prim(points), 18);
    }
}
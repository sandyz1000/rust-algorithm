
#![allow(unused)]

use std::collections::HashMap;
struct Solution;
type SparseMatrix = HashMap<i32, HashMap<i32, i32>>;

impl Solution {
    // pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    //     let mut ans: Vec<i32> = vec![];
        
    //     for i in 0..nums.len() {
    //         ans.push(nums[i] + ans.last().unwrap_or(&0));
    //     }

    //     ans
    // }

    /// Sparse Matrix Multiplication
    /// https://leetcode.com/problems/sparse-matrix-multiplication/
    ///
    /// Given two sparse matrices mat1 of size m x k and mat2 of size k x n, return the result 
    /// of mat1 x mat2. You may assume that multiplication is always possible.
    ///
    ///
    /// Example 1:
    /// -----------
    /// ```
    /// let mat1 = vec![vec![1,0,0],vec![-1,0,3]]; let mat2 = vec![vec![7,0,0],vec![0,0,0],vec![0,0,1]];
    /// let ans = [[7,0,0],[-7,0,3]];
    /// assert_eq!(Solution::multiply(mat1, mat2), ans);
    /// ```
    ///
    /// Example 2:
    /// -----------
    /// ``` 
    /// let mat1 = vec![vec![0]]; let mat2 = vec![vec![0]];
    /// assert_eq!(Solution::multiply(mat1, mat2), vec![vec![0]]);
    /// ```
    /// 
    /// Constraints:
    /// -----------
    /// - m == mat1.length
    /// - k == mat1[i].length == mat2.length
    /// - n == mat2[i].length
    /// - 1 <= m, n, k <= 100
    /// - -100 <= mat1[i][j], mat2[i][j] <= 100
    pub fn multiply(mat1: Vec<Vec<i32>>, mat2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n, p) = (mat1.len(), mat1[0].len(), mat2[0].len());

        let mut cache: HashMap<(i32, i32), i32> = HashMap::new();
        // Use hashmap to store key as index and value as cell value where value != 0
        let mut cache: SparseMatrix = HashMap::new();
        // Use hashmap to store key as index and value as cell value where value != 0
        for i in 0..mat1.len() {
            for j in 0..mat1[0].len() {
                if mat1[i][j] != 0 {
                    let entry: &mut HashMap<i32, i32> = cache.entry(i as i32).or_insert(HashMap::new());
                    entry.insert(j as i32, mat1[i][j]);
                }
            }
        }

        // The result matrix will have M x P dimension, 
        // where M = no of row mat1 and P = no of col mat2
        let mut result: Vec<Vec<i32>> = vec![vec![0; p]; m];

        for (i, row) in cache.iter() {
            for newc in 0..p {
                // Multiple any valid key in mat1 with mat2
                for (key2, value) in row.into_iter() {
                    result[*i as usize][newc] += value * mat2[newc as usize][*key2 as usize];
                }
            }
        }
        
        result
    }
}

mod test {
    use super::*;

    #[test]
    fn test_multiply() {
        let mat1 = vec![vec![1,0,0],vec![-1,0,3]]; 
        let mat2 = vec![vec![7,0,0],vec![0,0,0],vec![0,0,1]];
        let ans = [[7,0,0],[-7,0,3]];
        assert_eq!(Solution::multiply(mat1, mat2), ans);
    }

    #[test]
    fn test_multiple_single_value() {
        let mat1 = vec![vec![0]]; let mat2 = vec![vec![0]];
        assert_eq!(Solution::multiply(mat1, mat2), vec![vec![0]]);
    }

    #[test]
    fn test_use_single_value() {
        let mat1 = vec![vec![1,-5]];
        let mat2 = vec![vec![12],vec![-1]];
        assert_eq!(Solution::multiply(mat1, mat2), vec![vec![17]]);
    }
}
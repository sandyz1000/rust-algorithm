#![allow(unused)]

struct Solution;

type BoardType = Vec<Vec<String>>;

impl Solution {
    /// ## 51. N-Queens
    /// https://leetcode.com/problems/n-queens/
    ///
    ///
    /// The n-queens puzzle is the problem of placing n queens on an n x n chessboard such 
    /// that no two queens attack each other.
    ///
    /// Given an integer n, return all distinct solutions to the n-queens puzzle. You may 
    /// return the answer in any order.
    ///
    /// Each solution contains a distinct board configuration of the n-queens' placement, 
    /// where 'Q' and '.' both indicate a queen and an empty space, respectively.
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let n = 4
    /// let res = vec![
    /// vec![".Q..", "...Q", "Q...", "..Q."],
    /// vec!["..Q.","Q...","...Q",".Q.."]];
    /// assert_eq!(Solution::solve_n_queens(n), res);
    /// ```
    /// Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let n = 1; let res = vec![vec!["Q"]];
    /// assert_eq!(Solution::solve_n_queens(n), res);
    /// ```
    fn solve_n_queens(n: i32) -> BoardType {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_solve_n_queens() {
        let ans: Vec<Vec<String>> = vec![
            vec![".Q..", "...Q", "Q...", "..Q."].iter().map(|row| row.to_string()).collect(), 
            vec!["..Q.","Q...","...Q",".Q.."].iter().map(|row| row.to_string()).collect()
        ];
    
        let ans: Vec<Vec<String>> = vec![
            vec!["..Q.", "Q...", "...Q", ".Q.."].iter().map(|row| row.to_string()).collect(), 
            vec![".Q..", "...Q", "Q...", "..Q."].iter().map(|row| row.to_string()).collect()
        ];
    
        let n = 4;
        
        assert_eq!(Solution::solve_n_queens(n), ans);

        let ans = vec![vec!["Q".to_string()]];
        let n = 1;
    
        assert_eq!(Solution::solve_n_queens(n), ans);
    }
}
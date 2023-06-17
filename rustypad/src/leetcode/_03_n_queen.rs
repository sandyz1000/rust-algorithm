/* 
51. N-Queens
https://leetcode.com/problems/n-queens/


The n-queens puzzle is the problem of placing n queens on an n x n chessboard such 
that no two queens attack each other.

Given an integer n, return all distinct solutions to the n-queens puzzle. You may 
return the answer in any order.

Each solution contains a distinct board configuration of the n-queens' placement, 
where 'Q' and '.' both indicate a queen and an empty space, respectively.

Example 1:
----------
Input: n = 4
Output: 
[[".Q..", "...Q", "Q...", "..Q."],
["..Q.","Q...","...Q",".Q.."]]
Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above

Example 2:
----------
Input: n = 1
Output: [["Q"]]


 */
#![allow(unused)]

#[derive(Debug, Clone)]
struct  Solution;

type BoardType = Vec<Vec<String>>;

impl Solution {
    fn is_valid_pos(row: i32, col: i32, board: &Vec<Vec<char>>, n: i32) -> bool {
        // Check if queen present in upper diagonal
        let mut p1: i32 = row;
        let mut p2: i32 = col;
        
        while p1 >= 0 && p2 >= 0 {
            if board[p1 as usize][p2 as usize] == 'Q' {
                return false;
            }
            p1 -= 1;
            p2 -= 1;
        }
        
        // Check if queen present in lower diagonal
        let mut p1 = row;
        let mut p2 = col;

        while p1 < n && p2 >= 0 {
            if board[p1 as usize][p2 as usize] == 'Q' {
                return false;
            }
            p1 += 1;
            p2 -= 1;
        }
        
        // Check horizontal pos to the left
        let mut p2 = col;

        while p2 >= 0 {
            if board[row as usize][p2 as usize] == 'Q' {
                return false;
            }
            p2 -= 1
        }

        true
    }

    fn n_queen_utils(board: &mut Vec<Vec<char>>, ans: &mut BoardType, col: i32, n: i32) {
        if col >= n {
            let x: Vec<String> = board.iter()
                .map(|x| x.iter().collect()).collect();
            ans.push(x);
            return;
        }

        for row in 0..n {
            if Solution::is_valid_pos(row, col, board, n) {
                board[row as usize][col as usize] = 'Q';
                // Recurse for other valid position
                Solution::n_queen_utils(board, ans, col+1, n);
                // Backtrack
                board[row as usize][col as usize] = '.';
            }
        }
    }

    /// Iterate 
    /// Check if current row and col position is valid for current queen
    /// If not; then backtrack
    fn solve_n_queens(n: i32) -> BoardType {
        // Create a n x n grid. 
        let mut ans: BoardType = vec![];
        let mut board: Vec<Vec<char>> = vec![vec!['.' ; n as usize ]; n as usize];
        
        Solution::n_queen_utils(&mut board, &mut ans, 0, n);
        
        ans
    }
}

#[test]
fn test1() {
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
}

#[test]
fn test2() {
    let ans = vec![vec!["Q".to_string()]];
    let n = 1;

    assert_eq!(Solution::solve_n_queens(n), ans);
}
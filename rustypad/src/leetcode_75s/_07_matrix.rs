#![allow(unused)]

struct Solution;

impl Solution {
    
    /// ## 79. Word Search
    ///
    /// https://leetcode.com/problems/word-search/
    ///
    /// Given an m x n grid of characters board and a string word, return true if word exists in the grid.
    ///
    /// The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are 
    /// horizontally or vertically neighboring. The same letter cell may not be used more than once.
    ///
    ///  
    /// Example 1:
    /// ---------
    /// - Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
    /// - Output: true
    ///
    /// Example 2:
    /// ---------
    /// - Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
    /// - Output: true
    /// 
    /// Example 3:
    /// ---------
    /// - Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
    /// - Output: false
    ///  
    /// Constraints:
    /// -----------
    /// m == board.length
    /// n = board[i].length
    /// 1 <= m, n <= 6
    /// 1 <= word.length <= 15
    /// board and word consists of only lowercase and uppercase English letters.  
    ///
    /// Follow up: Could you use search pruning to make your solution faster with a larger board?
    ///
    /// ALGO:
    /// ------
    /// DFS and backtrack
    /// - start from board left most corner
    /// - increament word position + 1 if current word match the board element
    /// - add into path and return True if match
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        unimplemented!()
    }

    /// ## 73. Set Matrix Zeroes
    /// https://leetcode.com/problems/set-matrix-zeroes/
    ///
    /// Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's.
    ///
    /// You must do it in place.
    ///
    /// Example 1:
    /// -----------
    /// - Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
    /// - Output: [[1,0,1],[0,0,0],[1,0,1]]
    ///
    /// Example 2:
    /// -----------
    /// - Input: matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
    /// - Output: [[0,0,0,0],[0,4,5,0],[0,3,1,0]]
    ///
    /// Constraints:
    /// -----------
    /// - m == matrix.length
    /// - n == matrix[0].length
    /// - 1 <= m, n <= 200
    /// - -231 <= matrix[i][j] <= 231 - 1
    ///
    /// Follow up:
    /// ----------
    /// - A straightforward solution using O(mn) space is probably a bad idea.
    /// - A simple improvement uses O(m + n) space, but still not the best solution.
    /// - Could you devise a constant space solution?
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        unimplemented!()
    }
}

mod test {
    use super::*;

    fn vec_str_to_char(vec: Vec<Vec<&str>>) -> Vec<Vec<char>> {
        
        let ans = vec
            .into_iter()
            .map(
                |x| 
                x.into_iter().map(|x| x.chars().nth(0).unwrap()).collect() 
            ).collect();
        ans
    }

    #[test]
    fn test_word_search() {
        // Test case 1
        let board = vec![vec!["A","B","C","E"],vec!["S","F","C","S"],vec!["A","D","E","E"]];
        let word = "ABCCED".to_string();
        let board: Vec<Vec<char>> = vec_str_to_char(board);
        assert_eq!(Solution::exist(board, word), true);
        
        // Test case 2
        let board = vec![vec!["A","B","C","E"],vec!["S","F","C","S"],vec!["A","D","E","E"]];
        let word = "SEE".to_string();
        let board: Vec<Vec<char>> = vec_str_to_char(board);
        assert_eq!(Solution::exist(board, word), true);   
        
        // Test case 3
        let board = vec![vec!["A","B","C","E"],vec!["S","F","C","S"],vec!["A","D","E","E"]];
        let word = "ABCB".to_string();
        let board: Vec<Vec<char>> = vec_str_to_char(board);
        assert_eq!(Solution::exist(board, word), false);
    }

    #[test] 
    fn test_matrix_zeroes() {

    }

}
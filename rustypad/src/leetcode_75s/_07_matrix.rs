#![allow(unused)]

use std::collections::HashMap;

struct Solution;

struct Trie {
    children: HashMap<char, Trie>,
    word: Option<String>
}

impl Trie {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            word: None
        }
    }

    fn insert(&mut self, word: String) {
        let mut root = self;
        for c in word.chars() {
            if root.children.contains_key(&c) {
                root = root.children.get_mut(&c).unwrap();
            } else {
                root.children.insert(c, Trie::new());
                root = root.children.get_mut(&c).unwrap();
            }
        }
        root.word = Some(word);
    }
}
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
    /// ```
    /// let board = vec![vec!["A","B","C","E"],vec!["S","F","C","S"],vec!["A","D","E","E"]]; 
    /// let word = "ABCCED";
    /// assert_eq!(Solution::exist(board, word), true);
    /// ```
    /// 
    /// Example 2:
    /// ---------
    /// ```
    /// let board = vec![vec!["A","B","C","E"],vec!["S","F","C","S"],vec!["A","D","E","E"]]; 
    /// let word = "SEE";
    /// assert_eq!(Solution::exist(board, word), true);
    /// ```
    /// 
    /// Example 3:
    /// ---------
    /// ```
    /// let board = vec![vec!["A","B","C","E"],vec!["S","F","C","S"],vec!["A","D","E","E"]]; 
    /// let word = "ABCB";
    /// assert_eq!(Solution::exist(board, word), false);
    ///  ```
    /// 
    /// Constraints:
    /// -----------
    /// * m == board.length
    /// * n = board[i].length
    /// * 1 <= m, n <= 6
    /// * 1 <= word.length <= 15
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
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        // Time Complexity: O(N*3^L) where N is the number of cells 
        // in the board and L is the length of the word to be matched.
        fn get_neighbours(board: &Vec<Vec<char>>, r: i32, c: i32) -> Vec<(usize, usize)> {
            let mut ans = vec![];
            let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
            let (nrows, ncols) = (board.len() as i32, board[0].len() as i32);
            for (x, y) in directions {
                if (r + x > nrows -1 || r + x < 0) || ((c + y) > ncols - 1 || (c+y) < 0) {
                    continue;
                }
                ans.push(((r+x) as usize, (c+y) as usize));
            }

            ans
        }
        fn backtrack(
            board : &mut Vec<Vec<char>>, 
            row: usize, col: usize,
            idx: usize, 
            word: &Vec<char>
        ) -> bool {
            // Base case If idx == word.len(), then return true (every character match)
            if idx == word.len() {
                return true;
            }
            // Check if row and col is valid, if not return false
            if board[row][col] == '#' {
                return false
            }
            let temp = board[row][col];
            // Mark cell visited by mutating the board position with #
            board[row][col] = '#';

            // Get every neighbouring cell from this coordinate
            if temp == word[idx] {
                if idx == word.len() - 1 {
                    return true;
                }
                for (dx, dy) in get_neighbours(&board, row as i32, col as i32) {
                    if backtrack(board, dx, dy, idx + 1, word) {
                        return true;
                    }
                }
            }
            // If not true backtrack
            board[row][col] == temp;

            // If nothing match false
            false
        }

        let word: Vec<char> = word.chars().collect();
        for row in 0..board.len() {
            for col in 0..board[0].len() {
                if backtrack(&mut board, row, col, 0, &word) {
                    return true;
                }
            } 
        }
        false
    }

    fn convert_vec_vec_to_string(vec: Vec<Vec<&str>>) -> Vec<Vec<String>> {
        vec.into_iter()
            .map(|inner| inner.into_iter().map(|s| s.to_string())
            .collect()).collect()
    }

    /// ## 212. Word Search II
    /// https://leetcode.com/problems/word-search-ii/description/
    ///
    /// Given an m x n board of characters and a list of strings words, return all words on the board.
    ///
    /// Each word must be constructed from letters of sequentially adjacent cells, where adjacent cells 
    /// are horizontally or vertically neighboring. The same letter cell may not be used more than once in a word.
    ///
    /// Example 1:
    /// -----------
    /// ```
    /// let board = vec![vec!['o','a','a','n'],vec!['e','t','a','e'],vec!['i','h','k','r'],vec!['i','f','l','v']];
    /// let words = vec!["oath".to_string(),"pea".to_string(),"eat".to_string(),"rain".to_string()]
    /// let ans = vec!["eat","oath"]
    /// assert_eq!(Solution::find_words(board, words), ans)
    /// ```
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let board = vec![vec!['a','b'], vec!['c','d']]; let words = vec!["abcb".to_string()];
    /// let ans: Vec<String> = vec![]
    /// assert_eq!(Solution::find_words(board, words), ans)
    /// ```
    ///
    /// Constraints:
    /// ------------
    /// * m == board.length
    /// * n == board[i].length
    /// * 1 <= m, n <= 12
    /// * board[i][j] is a lowercase English letter.
    /// * 1 <= words.length <= 3 * 104
    /// * 1 <= words[i].length <= 10
    /// * words[i] consists of lowercase English letters.
    /// * All the strings of words are unique.
    /// 
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut matched_words: Vec<String> = Vec::new();
        let mut trie = Trie::new();
        for word in words.iter() {
            trie.insert(word.clone())
        }
        println!("Words: {:?}", words);
        // Iterate the each cell
        for row in 0..board.len() {
            for col in 0..board[0].len() {
                let letter = board[row][col];
                if trie.children.contains_key(&letter) {
                    Self::backtracking(&mut board, row as i32, col as i32, &mut trie, &mut matched_words);
                }
            }
        }

        matched_words
    }

    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    fn backtracking(
        board: &mut Vec<Vec<char>>,
        row: i32, col: i32, 
        parent: &mut Trie, 
        matched_words: &mut Vec<String>
    ) {
    
        // Look for current character in trie
        let letter = board[row as usize][col as usize];
        let curr_node = parent.children.get_mut(&letter).unwrap();
        // If word found add it to the matched_words
        if curr_node.word.is_some() {
            matched_words.push(curr_node.word.clone().unwrap());
        }

        // Mark this visited
        board[row as usize][col as usize] = '#';
        for (dx, dy) in Self::DIRECTIONS {
            let (n_row, n_col) = (dx + row, dy+ col);
            if n_row < 0 || n_row >= board.len() as i32 || n_col < 0 || n_col >= board[0].len() as i32 {
                continue;
            }

            if !curr_node.children.contains_key(&board[n_row as usize][n_col as usize]) {
                continue;
            }
            Self::backtracking(board, n_row, n_col, curr_node, matched_words);
        }

        // Backtrack
        board[row as usize][col as usize] = letter;
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
    /// ```
    /// let mut matrix = vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]];
    /// let ans = vec![vec![1,0,1], vec![0,0,0], vec![1,0,1]];
    /// Solution::set_zeroes(&mut matrix);
    /// assert_eq!(matrix, ans);
    /// ```
    /// 
    /// Example 2:
    /// -----------
    /// ```
    /// let mut matrix = vec![vec![0,1,2,0], vec![3,4,5,2], vec![1,3,1,5]];
    /// let ans = vec![vec![0,0,0,0],vec![0,4,5,0], vec![0,3,1,0]];
    /// Solution::set_zeroes(&mut matrix);
    /// assert_eq!(matrix, ans);
    /// ```
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

    /// ## 54. Spiral Matrix
    /// https://leetcode.com/problems/spiral-matrix/
    ///
    /// Given an m x n matrix, return all elements of the matrix in spiral order.
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let matrix = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    /// let ans = vec![1,2,3,6,9,8,7,4,5];
    /// assert_eq!(Solution::spiral_order(matrix), ans);
    /// ```
    /// 
    /// Example 2:
    /// ----------
    /// ```
    /// let matrix = vec![vec![1,2,3,4],vec![5,6,7,8], vec![9,10,11,12]];
    /// let ans = vec![1,2,3,4,8,12,11,10,9,5,6,7];
    /// assert_eq!(Solution::spiral_order(matrix), ans);
    /// ```
    /// 
    /// Constraints:
    /// ------------
    /// - m == matrix.length
    /// - n == matrix[i].length
    /// - 1 <= m, n <= 10
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![];
        let rows = matrix.len(); let cols = matrix[0].len();
        let (mut left, mut right) = (0, cols -1);
        let (mut up, mut down) = (0, rows -1);
        let mut size = 0;
        while size < rows * cols {
            // For top-left to top-right
            for i in left..right {
                ans.push(matrix[up][i]);
                size += 1;
            }
            // For top-right to bottom-right
            for i in up..down {
                ans.push(matrix[i][right]);
                size += 1;
            }

            // For bottom-right to bottom-left
            for i in (left+1..right+1).rev() {
                ans.push(matrix[down][i]);
                size += 1;
            } 

            // For bottom-left to top-left
            for i in (up+1..down+1).rev() {
                ans.push(matrix[i][left]);
                size += 1;
            }
            
            left += 1;
            right -= 1;
            up += 1;
            down -=1;
        }
        ans
    }

    /// ## 48. Rotate Image
    /// https://leetcode.com/problems/rotate-image/
    ///
    /// You are given an n x n 2D matrix representing an image, rotate the image by 90 degrees (clockwise).
    ///
    /// You have to rotate the image in-place, which means you have to modify the input 2D matrix directly. 
    /// DO NOT allocate another 2D matrix and do the rotation.
    ///
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let mut matrix = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    /// let ans = vec![vec![7,4,1], vec![8,5,2], vec![9,6,3]];
    /// Solution::rotate(&mut matrix)
    /// assert_eq!(matrix, ans);
    /// ```
    /// 
    /// Example 2:
    /// ----------
    /// ```
    /// let mut matrix = vec![vec![5,1,9,11],vec![2,4,8,10],vec![13,3,6,7],vec![15,14,12,16]]
    /// let ans = vec![vec![15,13,2,5],vec![14,3,4,1],vec![12,6,8,9],vec![16,7,10,11]]
    /// Solution::rotate(&mut matrix)
    /// assert_eq!(matrix, ans);
    /// ```
    /// 
    /// Constraints:
    /// -------------
    /// * n == matrix.length == matrix[i].length
    /// * 1 <= n <= 20
    /// * -1000 <= matrix[i][j] <= 1000
    ///
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        Solution::transpose(matrix);
        Solution::reverse_column(matrix);
    }

    fn transpose(matrix: &mut Vec<Vec<i32>>) {
        for i in 0..matrix.len() {
            for j in (i+1)..matrix[0].len() {
                let (v1, v2) = (matrix[i][j], matrix[j][i]);
                matrix[i][j] = v2;
                matrix[j][i] = v1;
            }
        }  
    } 

    fn reverse_column(matrix: &mut Vec<Vec<i32>>) {
        // For reversing the column j becomes n - j -1
        let ncols = matrix[0].len() as i32;
        let mut i = 0;
        while i < ncols / 2 {
            for r in 0..matrix.len() {
                let (v1, v2) = (matrix[r][i as usize], matrix[r][(ncols - i - 1) as usize]);
                matrix[r][i as usize] = v2;
                matrix[r][(ncols - i - 1) as usize] = v1;
            }
            i += 1;
        }
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
        let mut matrix = vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]];
        let ans = vec![vec![1,0,1], vec![0,0,0], vec![1,0,1]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, ans);

        let mut matrix = vec![vec![0,1,2,0], vec![3,4,5,2], vec![1,3,1,5]];
        let ans = vec![vec![0,0,0,0],vec![0,4,5,0], vec![0,3,1,0]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, ans);
    }

    #[test]
    fn test_spiral_order() {
        let matrix = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
        let ans = vec![1,2,3,6,9,8,7,4,5];
        assert_eq!(Solution::spiral_order(matrix), ans);

        let matrix = vec![vec![1,2,3,4],vec![5,6,7,8], vec![9,10,11,12]];
        let ans = vec![1,2,3,4,8,12,11,10,9,5,6,7];
        assert_eq!(Solution::spiral_order(matrix), ans);
    }

    #[test]
    fn test_rotate_image() {
        let mut matrix = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
        let ans = vec![vec![7,4,1], vec![8,5,2], vec![9,6,3]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, ans);

        let mut matrix = vec![vec![5,1,9,11],vec![2,4,8,10],vec![13,3,6,7],vec![15,14,12,16]];
        let ans = vec![vec![15,13,2,5],vec![14,3,4,1],vec![12,6,8,9],vec![16,7,10,11]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, ans);
    }
}
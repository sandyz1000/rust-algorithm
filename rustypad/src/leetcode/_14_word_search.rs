/* 

https://leetcode.com/problems/word-search/

Given an m x n grid of characters board and a string word, return true if word exists in the grid.

The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are 
horizontally or vertically neighboring. The same letter cell may not be used more than once.

ALGO:
DFS and backtrack
- start from board left most corner
- increament word position + 1 if current word match the board element
- add into path and return True if match
*/


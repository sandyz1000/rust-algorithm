/* 
Longest Valid Parentheses

# https://leetcode.com/problems/longest-valid-parentheses/description/

Given a string containing just the characters '(' and ')', find the length of the longest valid 
(well-formed) parentheses substring.

Example 1:

Input: s = "(()"
Output: 2
Explanation: The longest valid parentheses substring is "()".
Example 2:

Input: s = ")()())"
Output: 4
Explanation: The longest valid parentheses substring is "()()".
Example 3:

Input: s = ""
Output: 0

// Build graph and connect edges with single character removed
// BFS traversal for each node to verify it's validity 
*/
/* 
344. Reverse String

Write a function that reverses a string. The input string is given as an array of characters s.

You must do this by modifying the input array in-place with O(1) extra memory.

Example 1:
----------
Input: s = ["h","e","l","l","o"]
Output: ["o","l","l","e","h"]

Example 2:
----------
Input: s = ["H","a","n","n","a","h"]
Output: ["h","a","n","n","a","H"]
 */
#![allow(unused)]

struct Solution;

impl Solution {
    fn reverse_string(s: &mut Vec<char>) {
        // Two pointer solution with start and end pointer
        let (mut start, mut end): (u32, u32) = (0, s.len() as u32 - 1); 

        while start < end {
            s.swap(start as usize, end as usize);
            start += 1;
            end -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1(){
        let mut s: Vec<char> = vec!['h','e','l','l','o'];
        Solution::reverse_string(&mut s);
        let ans: Vec<char> = vec!['o','l','l','e','h'];
        assert_eq!(ans, s);
    }

    #[test]
    fn test2(){
        let mut s: Vec<char> = vec!['H','a','n','n','a','h'];
        Solution::reverse_string(&mut s);
        let ans: Vec<char> = vec!['h','a','n','n','a','H'];
        assert_eq!(ans, s);
    }

    #[test]
    fn test3() {
        let mut s = vec!['\"'];
        Solution::reverse_string(&mut s);
        let ans = s.clone();
        assert_eq!(ans, s);
    }
}
/* 

https://leetcode.com/problems/possible-bipartition/solutions/1773578/rust-bfs/?q=Rust&orderBy=most_relevant
886. Possible Bipartition

We want to split a group of n people (labeled from 1 to n) into two groups of any size. 
Each person may dislike some other people, and they should not go into the same group.

Given the integer n and the array dislikes where dislikes[i] = [ai, bi] indicates that 
the person labeled ai does not like the person labeled bi, return true if it is possible 
to split everyone into two groups in this way.

Example 1:
----------
Input: n = 4, dislikes = [[1,2],[1,3],[2,4]]
Output: true
Explanation: The first group has [1,4], and the second group has [2,3].

Example 2:
----------
Input: n = 3, dislikes = [[1,2],[1,3],[2,3]]
Output: false
Explanation: We need at least 3 groups to divide them. We cannot put them in two groups.
 
Constraints:
-----------
1 <= n <= 2000
0 <= dislikes.length <= 104
dislikes[i].length == 2
1 <= ai < bi <= n
All the pairs of dislikes are unique.

 */

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn possible_bipartition(n: i32, dislikes: Vec<[i32; 2]>) -> bool {
        false        
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let n = 3;
        let dislikes = vec![[1,2],[1,3],[2,3]];
        assert_eq!(Solution::possible_bipartition(n, dislikes), true);
    }

    #[test]
    fn test2() {
        let n = 3;
        let dislikes = vec![[1,2],[1,3],[2,3]];
        assert_eq!(Solution::possible_bipartition(n, dislikes), true);
    }

}
/*

https://leetcode.com/problems/cheapest-flights-within-k-stops/description/

Cheapest Flights Within K Stops

There are n cities connected by some number of flights. You are given an array
flights where flights[i] = [fromi, toi, pricei] indicates that there is a flight
from city fromi to city toi with cost pricei.

You are also given three integers src, dst, and k, return the cheapest price from
src to dst with at most k stops. If there is no such route, return -1.

Ex-1:
Input: n = 4, flights = [[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]],
src = 0, dst = 3, k = 1
Output: 700
Explanation:
The graph is shown above.
The optimal path with at most 1 stop from city 0 to 3 is marked in red and has
cost 100 + 600 = 700.
Note that the path through cities [0,1,2,3] is cheaper but is invalid because
it uses 2 stops.

Ex-2:
Input: n = 3, flights = [[0,1,100],[1,2,100],[0,2,500]], src = 0, dst = 2, k = 1
Output: 200
Explanation:
The graph is shown above.
The optimal path with at most 1 stop from city 0 to 2 is marked in red and has
cost 100 + 100 = 200

Ex-3:
Input: n = 3, flights = [[0,1,100],[1,2,100],[0,2,500]], src = 0, dst = 2, k = 0
Output: 500
Explanation:
The graph is shown above.
The optimal path with no stops from city 0 to 2 is marked in red and has cost 500.

 */

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};

type GraphType = HashMap<i8, Vec<(i8, i32)>>;
type State = (i8, i8);

struct Solution;

impl Solution {
    /// Create a distance map with initialized to infinty
    /// BFS till k depth and update the distance
    /// Finally return distance of target
    #[allow(dead_code)]
    fn find_cheapest_price(n: i32, flights: Vec<(i8, i8, i32)>, src: i8, dst: i8, k: i32) -> i32 {
        // Build weighted graph
        let mut graph: GraphType = HashMap::new();
        for (s, d, w) in flights {
            match graph.contains_key(&s) {
                true => {
                    let c = graph.get_mut(&s).unwrap();
                    c.push((d, w));
                }
                false => {
                    graph.insert(s, vec![(d, w)]);
                }
            }
        }

        let mut distance: Vec<i32> = vec![f32::INFINITY as i32; n as usize];
        distance[src as usize] = 0;

        let mut __q: VecDeque<(i32, i8)> = VecDeque::new();
        __q.push_back((0, src));

        let mut level = k.clone();

        while !__q.is_empty() && level != 0 {
            let mut q_size = __q.len();
            while q_size > 0 {
                let (ndist, node) = __q.pop_front().unwrap();
                if let Some(children) = graph.get(&node) {
                    for (conn, w) in children.iter() {
                        let cdist = *w + (ndist);
                        if cdist < distance[*(conn) as usize] {
                            distance[*(conn) as usize] = cdist;
                            __q.push_back((cdist, *conn));
                        }
                    }
                }
                q_size -= 1;
            }
            level -= 1
        }

        distance[dst as usize]
    }

    #[allow(dead_code)]
    fn find_cheapest_price_with_min_heap(
        n: i32,
        flights: Vec<(i8, i8, i32)>,
        src: i8,
        dst: i8,
        k: i8,
    ) -> i32 {
        // Build weighted graph

        let mut graph: GraphType = HashMap::new();
        for (s, d, w) in flights {
            match graph.contains_key(&s) {
                true => {
                    let node = graph.get_mut(&s).unwrap();
                    node.push((d, w));
                }
                false => {
                    graph.insert(s, vec![(d, w)]);
                }
            }
        }

        let mut distances: Vec<i32> = vec![(f32::INFINITY) as i32; n as usize];
        distances[src as usize] = 0;

        let mut __q: BinaryHeap<(Reverse<i32>, i8, i8)> = BinaryHeap::new();
        __q.push((Reverse(0), 0, src)); // min-heap (distance, depth, node)

        while !__q.is_empty() {
            if let Some((Reverse(distance), depth, node)) = __q.pop() {
                if let Some(connection) = graph.get(&node) {
                    for (conn, w) in connection {
                        // If distance from source is less than prev distance
                        let cdist = *w + distance;
                        if (cdist as i32) < distances[*(conn) as usize] && depth <= k {
                            distances[*(conn) as usize] = cdist;
                            __q.push((Reverse(cdist), depth + 1, *conn))
                        }
                    }
                }
            }
        }

        distances[dst as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        println!(">>> Test-1 >>>");
        let n = 4;
        let flights: Vec<(i8, i8, i32)> = vec![
            (0, 1, 100),
            (1, 2, 100),
            (2, 0, 100),
            (1, 3, 600),
            (2, 3, 200),
        ];
        let (src, dst, k) = (0, 3, 1);
        let res = Solution::find_cheapest_price_with_min_heap(n, flights, src, dst, k);
        assert_eq!(res, 700);
    }

    #[test]
    fn test2() {
        println!(">>> Test-2 >>>");
        let n = 3;
        let flights: Vec<(i8, i8, i32)> = vec![(0, 1, 100), (1, 2, 100), (0, 2, 500)];
        let (src, dst, k) = (0, 2, 1);
        let res = Solution::find_cheapest_price_with_min_heap(n, flights, src, dst, k);
        assert_eq!(res, 200);
    }

    #[test]
    fn test3() {
        println!(">>> Test-3 >>>");
        let n = 3;
        let flights: Vec<(i8, i8, i32)> = vec![(0, 1, 100), (1, 2, 100), (0, 2, 500)];
        let (src, dst, k) = (0, 2, 0);
        let res = Solution::find_cheapest_price_with_min_heap(n, flights, src, dst, k);
        assert_eq!(res, 500);
    }
}

/* 

https://www.geeksforgeeks.org/prims-minimum-spanning-tree-mst-greedy-algo-5/

http://www.geeksforgeeks.org/greedy-algorithms-set-5-prims-mst-for-adjacency-list-representation/


def prim_mst(X: typing.List[typing.Tuple]):
    graph = defaultdict(list)
    for src, dst, wt in X:
        graph[src].append((wt, dst))
        graph[dst].append((wt, src))
    min_heap = graph[0]
    heapq.heapify(min_heap)
    parent = np.full((len(graph), ), -1, dtype=int)
    visited = np.zeros((len(graph), ), int)
    count = 0

    # iterate min-heap
    while min_heap:
        wt, node = heapq.heappop(min_heap)
        if not visited[node]:
            visited[node] = 1
            for conn in graph[node]: 
                heapq.heappush(min_heap, conn)
            parent[conn[1]] = node
        if count >= len(graph): break 

*/


struct Solution;

impl Solution {
    fn prim_mst(graph: Vec<(i32, i32, i32)>) {

    }
}

#[test]
fn test() {
    let graph: Vec<(i32, i32, i32)> = vec![
        (0, 1, 4), (0, 7, 8), (1, 2, 8), 
        (1, 7, 11), (2, 3, 7), (2, 8, 2),
        (2, 5, 4), (3, 4, 9), (3, 5, 14), 
        (4, 5, 10), (5, 6, 2), (6, 7, 1), 
        (6, 8, 6), (7, 8, 7)
    ];
    
    Solution::prim_mst(graph)

}

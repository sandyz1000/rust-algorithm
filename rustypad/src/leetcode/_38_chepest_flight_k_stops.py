"""
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
"""

from collections import defaultdict, deque
from typing import List
import heapq


# Create a distance map with initialized to infinty
# BFS till k depth and update the distance
# Finally return distance of target
def findCheapestPrice(
    n: int,
    flights: List[List[int]],
    src: int, dst: int, k: int
) -> int:
    # Build weighted graph
    graph = defaultdict(dict)
    for s, d, w in flights:
        graph[s][d] = w

    distance = [float("inf")] * n
    distance[src] = 0
    __q = deque([(0, src)])
    while __q and k > -1:
        q_size = len(__q)
        while q_size:
            ndist, node = __q.popleft()

            for conn, w in graph[node].items():
                # If distance from source is 
                cdist = w + ndist
                if cdist < distance[conn]:
                    distance[conn] = cdist
                    __q.append((cdist, conn))

            q_size -= 1

        k -= 1

    return distance[dst]


def findCheapestPriceWithMinHeap(
    n: int,
    flights: List[List[int]],
    src: int, dst: int, k: int
) -> int:
    # Build weighted graph
    graph = defaultdict(dict)
    for s, d, w in flights:
        graph[s][d] = w

    distance = [float("inf")] * n
    distance[src] = 0
    __q = [(0, 0, src)]  # min-heap (distance, depth, node)
    while __q:
        ndist, depth, node = heapq.heappop(__q)

        for conn, w in graph[node].items():
            # If distance from source is less than prev distance
            cdist = w + ndist
            if cdist < distance[conn] and depth <= k:
                distance[conn] = cdist
                heapq.heappush(__q, (cdist, depth + 1, conn))

    return distance[dst]


def cli_main():
    print(">>> Test-1 >>>")
    N = 4
    flights = [[0, 1, 100], [1, 2, 100], [2, 0, 100], [1, 3, 600], [2, 3, 200]]
    src, dst, k = 0, 3, 1
    res = findCheapestPriceWithMinHeap(N, flights, src, dst, k)
    assert res == 700

    print(">>> Test-2 >>>")
    N = 3
    flights = [[0, 1, 100], [1, 2, 100], [0, 2, 500]]
    src, dst, k = 0, 2, 1
    res = findCheapestPriceWithMinHeap(N, flights, src, dst, k)
    assert res == 200
    
    print(">>> Test-3 >>>")
    N = 3
    flights = [[0, 1, 100], [1, 2, 100], [0, 2, 500]]
    src, dst, k = 0, 2, 0
    res = findCheapestPriceWithMinHeap(N, flights, src, dst, k)
    assert res == 500


if __name__ == "__main__":
    cli_main()

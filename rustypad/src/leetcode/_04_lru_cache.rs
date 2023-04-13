/* 
# https://leetcode.com/problems/lru-cache/description/
146. LRU Cache

Design a data structure that follows the constraints of a Least Recently Used (LRU) cache.

Implement the LRUCache class:

- LRUCache(int capacity) Initialize the LRU cache with positive size capacity.
- int get(int key) Return the value of the key if the key exists, otherwise return -1.
- void put(int key, int value) Update the value of the key if the key exists.
 Otherwise, add the key-value pair to the cache. If the number of keys exceeds the 
 capacity from this operation, evict the least recently used key.

The functions get and put must each run in O(1) average time complexity.


Example 1:
---------
Input
["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
[[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
Output
[null, null, null, 1, null, -1, null, -1, 3, 4]

Explanation
----------
LRUCache lRUCache = new LRUCache(2);
lRUCache.put(1, 1); // cache is {1=1}
lRUCache.put(2, 2); // cache is {1=1, 2=2}
lRUCache.get(1);    // return 1
lRUCache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
lRUCache.get(2);    // returns -1 (not found)
lRUCache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
lRUCache.get(1);    // return -1 (not found)
lRUCache.get(3);    // return 3
lRUCache.get(4);    // return 4
 

Constraints:
-----------
1 <= capacity <= 3000
0 <= key <= 104
0 <= value <= 105
At most 2 * 105 calls will be made to get and put.

 */
#![allow(unused_variables)]
#![allow(dead_code)]


use std::collections::HashMap;
use std::cell::RefCell;


#[derive(Default, Debug)]
struct  Node;

#[derive(Default, Debug)]
struct List;

type MapType = HashMap<i32, Option<RefCell<Node>>>;

#[derive(Default, Debug)]
struct LRUCache {
    hash_map: MapType,
    list: List,
    length: usize,
    capacity: usize,
}

impl LRUCache {
    
    fn new(capacity: i32) -> Self {
        // LRUCache {

        // }
        todo!()
    }

    #[allow(dead_code)]
    fn put(&mut self, key: i8, value: i8) {
        todo!()
    }

    #[allow(dead_code)]
    fn get(&self, key: i8) -> i8 {
        todo!()
    }
}


 #[test]
 fn test() {
     let mut obj = LRUCache::new(2);
     assert_eq!(obj.get(2), -1);
     obj.put(2, 6);
     assert_eq!(obj.get(1), -1);
     obj.put(1, 5);
     obj.put(1, 2);
     assert_eq!(obj.get(1), 2);
     assert_eq!(obj.get(2), 6);
 
     let mut obj = LRUCache::new(3);
     obj.put(1, 1);
     obj.put(2, 2);
     obj.put(3, 3);
     obj.put(4, 4);
     assert_eq!(obj.get(4), 4);
     assert_eq!(obj.get(3), 3);
     assert_eq!(obj.get(2), 2);
     assert_eq!(obj.get(1), -1);
     obj.put(5, 5);
     assert_eq!(obj.get(1), -1);
     assert_eq!(obj.get(2), 2);
     assert_eq!(obj.get(3), 3);
     assert_eq!(obj.get(4), -1);
     assert_eq!(obj.get(5), 5);
 }
 
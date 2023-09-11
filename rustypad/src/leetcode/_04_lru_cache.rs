#![allow(unused_variables)]
#![allow(dead_code)]


use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;


/// ## 146. LRU Cache
/// https://leetcode.com/problems/lru-cache/description/
///
/// Design a data structure that follows the constraints of a Least Recently Used (LRU) cache.
///
/// Implement the LRUCache class:
///
/// - LRUCache(int capacity) Initialize the LRU cache with positive size capacity.
/// - int get(int key) Return the value of the key if the key exists, otherwise return -1.
/// - void put(int key, int value) Update the value of the key if the key exists.
///  Otherwise, add the key-value pair to the cache. If the number of keys exceeds the 
///  capacity from this operation, evict the least recently used key.
///
/// The functions get and put must each run in O(1) average time complexity.
///
///
/// Example 1:
/// ---------
/// - Input
/// ["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
/// [[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
/// - Output
/// [null, null, null, 1, null, -1, null, -1, 3, 4]
///
/// Explanation
/// ----------
/// LRUCache lRUCache = new LRUCache(2);
/// lRUCache.put(1, 1); // cache is {1=1}
/// lRUCache.put(2, 2); // cache is {1=1, 2=2}
/// lRUCache.get(1);    // return 1
/// lRUCache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
/// lRUCache.get(2);    // returns -1 (not found)
/// lRUCache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
/// lRUCache.get(1);    // return -1 (not found)
/// lRUCache.get(3);    // return 3
/// lRUCache.get(4);    // return 4
///  
/// Constraints:
/// -----------
/// * 1 <= capacity <= 3000
/// * 0 <= key <= 104
/// * 0 <= value <= 105
///
/// At most 2 * 105 calls will be made to get and put.
#[derive(Debug)]
struct LRUCache {
    map: HashMap<i32, NodeRef>,
    lru: DoubleListNode,
    cap: usize
}


#[derive(Debug)]
struct ListNode {
    key: i32,
    value: i32,
    prev: Option<NodeRef>,
    next: Option<NodeRef>,
}

impl ListNode {
    pub fn new(key: i32, value: i32) -> Self {
        Self { key, value, prev: None, next: None }
    }
}

#[derive(Debug)]
struct DoubleListNode {
    head: Option<NodeRef>,
    tail: Option<NodeRef>,
}

type NodeRef = Rc<RefCell<ListNode>>;
type MapType = HashMap<i32, NodeRef>;


impl DoubleListNode {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None
        }
    }

    fn get_head(&self) -> Option<NodeRef> {
        if self.head.is_none() {
            None
        } else {
            Some(self.head.as_ref().unwrap().clone())
        }
    }

    fn get_tail(&self) -> Option<NodeRef> {
        if self.tail.is_none() {
            None
        } else {
            Some(self.tail.as_ref().unwrap().clone())
        }
    }

    pub fn add_front_node(&mut self, node: NodeRef) {
        let head: Option<NodeRef> = self.get_head();
        if head.is_some() {
            head.as_ref().unwrap().borrow_mut().prev = Some(node.clone());
        }

        node.borrow_mut().prev = None;
        node.borrow_mut().next = head;

        self.head.replace(node);
    }

    pub fn add_back_node(&mut self, node: NodeRef) {
        let tail: Option<NodeRef> = self.get_tail();
        if tail.is_some() {
            tail.as_ref().unwrap().borrow_mut().next = Some(node.clone());
        }

        node.borrow_mut().prev = tail;
        node.borrow_mut().next = None;

        self.tail.replace(node);
    }

    pub fn remove(&mut self, target: NodeRef) {
        let prev: Option<NodeRef> = target.borrow().prev.clone();
        let next: Option<NodeRef> = target.borrow().next.clone();
        
        match (prev, next) {
            (Some(prev), Some(next)) => {
                prev.borrow_mut().next = Some(next.clone());
                next.borrow_mut().prev = Some(prev);
            },
            (Some(prev), None) => {
                // tail case
                prev.borrow_mut().next.take();
                self.tail.replace(prev);
            },
            (None, Some(next)) => {
                // head case
                next.borrow_mut().prev.take();
                self.head.replace(next);
            },
            (None, None) => {
                // singal node case
                self.head.take();
                self.tail.take();
            }
        }
    }

    // This will move the target to the head
    pub fn move_head(&mut self, target: Rc<RefCell<ListNode>>) {
        if !Rc::ptr_eq(self.get_head().as_ref().unwrap(), &target) {
            self.remove(target.clone());
            self.add_front_node(target);
        }
    }

    pub fn move_tail(&mut self, target: Rc<RefCell<ListNode>>) {
        if !Rc::ptr_eq(self.get_tail().as_ref().unwrap(), &target) {
            self.remove(target.clone());
            self.add_back_node(target);
        }
    }
}


impl LRUCache {
    
    fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::new(),
            lru: DoubleListNode::new(),
            cap: capacity as usize,
        }
    }

    #[allow(dead_code)]
    fn put(&mut self, key: i32, value: i32) {
        // Look for the  key  if exist and replace it with the new value
        // Otherwise, add the key-value pair
        let node: NodeRef = if self.map.contains_key(&key) {
            let node: &NodeRef = self.map.get(&key).unwrap();
            node.borrow_mut().value = value;
            self.lru.move_head(node.clone());
            node.clone()
        } else {
            // Add the key-value pair and move to the head
            // Look if the cache is full
            let node: NodeRef = Rc::new(RefCell::new(ListNode::new(key, value)));
            if self.map.len()==self.cap {
                let tail: NodeRef = self.lru.get_tail().as_ref().unwrap().clone();
                self.map.remove(&tail.as_ref().borrow().key);
                self.lru.remove(tail);

            }
            self.map.insert(key, node.clone());
            self.lru.add_front_node(node.clone());
            node
        };
        if self.lru.tail.is_none() {
            self.lru.add_back_node(node);
        }
    }

    #[allow(dead_code)]
    fn get(&mut self, key: i32) -> i32 {
        // Get the target node and move to the head
        if self.map.contains_key(&key) {
            let node: &NodeRef = self.map.get(&key).unwrap();
            self.lru.move_head(node.clone());
            node.as_ref().borrow().value
        } else {
            -1
        }
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
 
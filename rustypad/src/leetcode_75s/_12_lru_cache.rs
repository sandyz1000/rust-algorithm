#![allow(unused)]
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

type NodeRef = Rc<RefCell<LinkNode>>;

struct LinkNode {
    key: i32,
    value: i32,
    next: Option<NodeRef>,
    prev: Option<NodeRef>
}

struct DoubleLinkedList {
    head: Option<NodeRef>,
    tail: Option<NodeRef>,
}

struct LRUCache {
    capacity: i32,
    map: HashMap<i32, NodeRef>,
    dll: DoubleLinkedList
}

impl DoubleLinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None
        }
    }
    
    /// Get the head node
    fn get_head(&self) -> Option<NodeRef> {
        if self.head.is_none() {
            return None;
        }
        Some(self.head.as_ref().unwrap().clone())
    }

    /// Get the tail node
    fn get_tail(&self) -> Option<NodeRef> {
        if self.tail.is_none() {
            return None;
        }
        Some(self.tail.as_ref().unwrap().clone())
    }

    /// This will move the target to the head 
    fn move_to_head(&mut self, target: NodeRef) -> Option<NodeRef> {
        // If the target is the head, do nothing
        // Otherwise, move the target to the head
        if !Rc::ptr_eq(self.head.as_ref().unwrap(), &target) {
            self.remove(target.clone());
            self.add_front_node(target);
        }
        self.get_head()
    }

    /// This will move the target to the tail
    fn move_to_tail(&mut self, target: NodeRef) -> Option<NodeRef> {
        // If the target is the tail, do nothing
        // Otherwise, move the target to the tail
        if !Rc::ptr_eq(self.tail.as_ref().unwrap(), &target) {
            self.remove(target.clone());
            self.add_back_node(target);
        }
        self.get_tail()
    }

    /// Add node to the front
    fn add_front_node(&mut self, node: NodeRef) {
        // Make the previous of head to new node
        let head: Option<NodeRef> = self.get_head();
        if head.is_some() {
            head.as_ref().unwrap().borrow_mut().prev = Some(node.clone());
        }
        // Fix the new node next and prev pointer
        node.borrow_mut().next = head;
        node.borrow_mut().prev = None;
        
        // Replace the head with new node
        self.head.replace(node);
    }

    /// Add node to the back
    fn add_back_node(&mut self, node: NodeRef) {
        let tail: Option<NodeRef> = self.get_tail();
        if tail.is_some() {
            tail.as_ref().unwrap().borrow_mut().next = Some(node.clone());
        }
        // Fix the new node next and prev pointer
        node.borrow_mut().prev = tail;
        node.borrow_mut().next = None;
        
        // Replace the tail with new node
        self.tail.replace(node);
    }

    fn remove(&mut self, node: NodeRef) {
        // Remove the node, the node prev and next have 4 cases
        let next: Option<NodeRef> = node.borrow().next.clone();
        let prev: Option<NodeRef> = node.borrow().prev.clone();
        match (next, prev) {
            // - If the node is in between
            (Some(next), Some(prev)) => {
                prev.borrow_mut().next = Some(next.clone());
                next.borrow_mut().prev = Some(prev.clone());
            }
            // - If the node is head node
            (Some(next), None) => {
                // This is the head case
                next.borrow_mut().prev.take();
                self.head.replace(next.clone());
            }
            // - If the node is tail node
            (None, Some(prev)) => {
                // This is the tail case
                prev.borrow_mut().next.take();
                self.tail.replace(prev.clone());
            }
            // - If the DLL has only one node
            (None, None) => {
                self.head.take();
                self.tail.take();
            }
        }
    }
}

impl LRUCache {

    fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::new(),
            dll: DoubleLinkedList::new(),
            capacity
        }
    }
    
    /// Get the value an move to the head
    /// If not found, return -1
    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key) {
            self.dll.move_to_head(node.clone());
            return node.borrow().value;
        }
        -1
    }
    
    /// Look if there is key in the cache
    /// If found, update the value and move to the head
    /// Otherwise, add the key-value pair 
    fn put(&mut self, key: i32, value: i32) {
        
        let node: NodeRef = if self.map.contains_key(&key) {
            let node: &NodeRef = self.map.get(&key).unwrap();
            node.borrow_mut().value = value;
            self.dll.move_to_head(node.clone());
            node.clone()
        } else {
            // If the cache is full, then pop from the tail
            if self.capacity == self.map.len() as i32 {
                let tail: NodeRef = self.dll.get_tail().unwrap();
                self.map.remove(&tail.as_ref().borrow().key);
                self.dll.remove(tail);
            } 
            // Add the key-value pair and add a front node
            let node: NodeRef = Rc::new(RefCell::new(LinkNode{ key, value, next: None, prev: None }));
            self.map.insert(key, node.clone());
            self.dll.add_front_node(node.clone());
            node
        };
        
        // Add to the tail if the DLL is empty
        if self.dll.tail.is_none() {
            self.dll.add_back_node(node);
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
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



}
#![allow(unused)]

pub struct UnionFind {
    parent: Vec<usize>,
    group: usize,
    n: usize,
}

impl UnionFind {
    /// Creates a new [`UnionFind`].
    pub fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        UnionFind {  
            parent, group: n, n
        }
    }

    pub fn find(&mut self, i: usize) -> usize {
        let j = self.parent[i];
        if i == j {
            return i;
        } 
        // Recursively find the parent of node in the heirarchy chain
        self.parent[i] = self.find(j);
        self.parent[i]
    }

    pub fn union(&mut self, mut i: usize, mut j: usize) -> bool {
        i = self.find(i);
        j = self.find(j);
        if i != j {
            self.parent[j] = i;
            self.group -= 1;
            true
        } else {
            false
        }
    }

    pub fn group(&self) -> usize {
        self.group
    }

    pub fn size(&self) -> usize {
        self.n
    }
}
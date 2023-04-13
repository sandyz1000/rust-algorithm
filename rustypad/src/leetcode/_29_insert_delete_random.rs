/*
https://leetcode.com/problems/insert-delete-getrandom-o1-duplicates-allowed/description/

Insert Delete GetRandom O(1) - Duplicates allowed

RandomizedCollection is a data structure that contains a collection of numbers,
possibly duplicates (i.e., a multiset). It should support inserting and removing
specific elements and also reporting a random element.

Implement the RandomizedCollection class:

- RandomizedCollection() Initializes the empty RandomizedCollection object.
bool insert(int val) Inserts an item val into the multiset, even if the item is
already present. Returns true if the item is not present, false otherwise.

- bool remove(int val) Removes an item val from the multiset if present. Returns
true if the item is present, false otherwise. Note that if val has multiple
occurrences in the multiset, we only remove one of them.

- int getRandom() Returns a random element from the current multiset of elements.
The probability of each element being returned is linearly related to the number
of the same values the multiset contains.

You must implement the functions of the class such that each function works on average
O(1) time complexity.

Note: The test cases are generated such that getRandom will only be called if there is
at least one item in the RandomizedCollection.


Example 1:
----------
Input
["RandomizedCollection", "insert", "insert", "insert", "getRandom", "remove", "getRandom"]
[[], [1], [1], [2], [], [1], []]
Output
[null, true, false, true, 2, true, 1]

Explanation
RandomizedCollection randomizedCollection = new RandomizedCollection();
randomizedCollection.insert(1);   // return true since the collection does not contain 1.
                                  // Inserts 1 into the collection.
randomizedCollection.insert(1);   // return false since the collection contains 1.
                                  // Inserts another 1 into the collection. Collection now contains [1,1].
randomizedCollection.insert(2);   // return true since the collection does not contain 2.
                                  // Inserts 2 into the collection. Collection now contains [1,1,2].
randomizedCollection.getRandom(); // getRandom should:
                                  // - return 1 with probability 2/3, or
                                  // - return 2 with probability 1/3.
randomizedCollection.remove(1);   // return true since the collection contains 1.
                                  // Removes 1 from the collection. Collection now contains [1,2].
randomizedCollection.getRandom(); // getRandom should return 1 or 2, both equally likely.


Constraints:
------------
-231 <= val <= 231 - 1
At most 2 * 105 calls in total will be made to insert, remove, and getRandom.
There will be at least one element in the data structure when getRandom is called.


 */

use std::{collections::HashMap, mem::swap, ops::Deref};

#[derive(Default, Clone)]
struct RandomizedCollection<'a> {
    arr: Vec<&'a str>,
    multiset: HashMap<i32, Vec<i32>>,
}

impl<'a> RandomizedCollection<'a> {
    /// - For multiset, create a dictionary with item and list
    /// - Create another arr to store the value, useful for getRandom in O(1) time
    #[allow(dead_code)]
    fn new(arr: Vec<&'a str>) -> Self {
        let multiset = HashMap::new();
        RandomizedCollection { arr, multiset }
    }

    /// Inserts an item val into the multiset, even if the item is already present.
    /// Returns true if the item is not present, false otherwise.
    #[allow(dead_code)]
    fn insert(&self, val: i32) -> bool {
        let present = self.multiset.contains_key(&val);
        // # Create value in arr if not present and save index
        // self.arr.append(val)
        // self.multiset[val].append(len(self.arr) - 1)
        present
    }

    /// Removes an item val from the multiset if present.
    /// Returns true if the item is present, false otherwise.

    /// Note that if val has multiple occurrences in the multiset, we only remove one of them.
    #[allow(dead_code)]
    fn remove(&mut self, val: i32) -> bool {
        let present = self.multiset.contains_key(&val);
        if !present {
            return false;
        }

        // Pop index from multiset and remove
        if let Some(item) = self.multiset.get_mut(&val) {
            let index: usize = item.pop().unwrap() as usize;
            // Swap with the last element of the arr, and remove last
            let mut x = self.arr[index as usize];
            let mut y = self.arr[self.arr.len() - 1];
            swap(&mut x, &mut y);
            self.arr.pop();
        }

        present
    }

    #[allow(dead_code)]
    fn get_random(&self) -> i32 {
        // let x = self.arr.choose(&mut rand::thread_rng()).unwrap();
        // *x
        todo!()
    }
}

// const SOME_CONSTANT_ONE: &str = "Test constant 1";
// const SOME_CONSTANT_TWO: &str = "Test constant 2";

#[test]
fn test_lifetime() {
    let vec_slice_first = vec![1, 2, 3, 4, 5];
    let vec_slice_sec = vec![6, 7, 8, 9, 10];
    let return_vec = get_vec_slice(&vec_slice_first, &vec_slice_sec);

    let str_1 = "Hello";
    let str_2 = "World!!!!";
    let x = *get_smaller(&str_1, &str_2);
    println!("The smaller length b/w two string is {:?}", x);
}

#[allow(dead_code)]
fn get_vec_slice<'a, T>(param_1: &'a T, param_2: &'a T) -> &'a T {
    todo!()
}

#[allow(dead_code)]
fn get_smaller<'a, T: PartialOrd>(param_1: &'a T, param_2: &'a T) -> &'a T {
    if param_1 > param_2 {
        param_1
    } else {
        param_2
    }
}

#[test]
fn test() {
    let func = vec![
        "insert",
        "insert",
        "insert",
        "getRandom",
        "remove",
        "getRandom",
    ];
    let rc = RandomizedCollection::new(func);
    // output = [getattr(rc, f) for f in func]
    // println!(output)
}

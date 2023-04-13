// 

use std::collections::HashMap;

struct Solution;

impl Solution {
    /// - Iterate nums and check if target - num contain in hashmap
    /// - return indices of two number
    #[allow(dead_code)]
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm: HashMap<i32, usize> = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            let prev = target - *num;
            if hm.contains_key(&prev) {
                return vec![hm[&prev] as i32, index as i32];
            }

            hm.insert(*num, index);
        }
        vec![]
    }
}

#[test]
fn test_string_ref() {
    // Rust &str vs String working example for understanding
        
    let foo: &str = "hello";
    let bar: String = foo.to_string();
    let baz: &str = &bar;
    let x: bool = bar.eq(baz);
    
    // This will be a copy of baz string
    let mut foobar: String = baz.to_owned();
    foobar = foobar + " " + "world";

    println!("The value of x: {} ", x);
    println!("The value of foo: {} ", foo);
    println!("The value of foobar: {} ", foobar);
}

#[test]
fn test() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
}

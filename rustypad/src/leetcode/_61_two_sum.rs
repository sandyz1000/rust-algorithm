#![allow(unused)]
use std::collections::HashMap;

struct Solution;

impl Solution {
    /// - Iterate nums and check if target - num contain in hashmap
    /// - return indices of two number
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm: HashMap<i32, usize> = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            let prev = target - *num;
            if hm.contains_key(&prev) {
                return vec![hm[&prev] as i32, index as i32];
            }

            hm.insert(num.clone(), index);
        }
        vec![]
    }

    /// - Two pointer O(n*logn)
    /// - Sort the vector and then take left and right pointer
    /// - Take the current sum and check if it is < target or > target
    /// - If it is < target, move left pointer to the next number
    /// - If it is > target, move right pointer to the next number
    /// - return indices of two number
    fn two_sum_two_pointer(nums: Vec<i32>, target: i32) -> Vec<i32> {
        /* 
        Equivalent python code
        num_indices = list(map(lambda x: (x[0], x[1]), enumerate(nums)))
        nums_indices = sorted(num_indices, key=lambda x: x[1])
         */
        let mut nums_indices: Vec<(usize, &i32)> = nums.iter().enumerate().collect::<Vec<(usize, &i32)>>();
        nums_indices.sort_by_key(|&(_, num)| num);
        // nums_indices.sort_by(|x, y| x.1.cmp(y.1));
        let mut left: i32 = 0;
        let mut right: i32 = (nums_indices.len() - 1) as i32;
        while left < right {
            let current = nums_indices[left as usize].1 + nums_indices[right as usize].1;
            if current == target {
                let (l, r) = (nums_indices[left as usize].0, nums_indices[right as usize].0);
                return vec![l as i32, r as i32];
            }
            if current < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
        vec![]
    }
}

mod tests {
    use super::*;
    
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
        assert_eq!(Solution::two_sum_two_pointer(nums, target), vec![0, 1]);
    }
    
    #[test]
    fn test_2() {
        let nums = vec![3,2,4];
        let target = 6;
        assert_eq!(Solution::two_sum_two_pointer(nums, target), vec![1, 2]);
    }

}
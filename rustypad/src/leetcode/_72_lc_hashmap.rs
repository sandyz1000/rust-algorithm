#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    /// Given an integer array arr, count how many elements x there are, such that x + 1 is also in arr.
    /// If there are duplicates in arr, count them separately.
    ///
    /// Example 1:
    /// ----------
    /// Input: arr = [1,2,3]
    /// Output: 2
    /// Explanation: 1 and 2 are counted cause 2 and 3 are in arr.
    ///
    /// Example 2:
    /// ----------
    /// Input: arr = [1,1,3,3,5,5,7,7]
    /// Output: 0
    /// Explanation: No numbers are counted, cause there is no 2, 4, 6, or 8 in arr.
    ///
    /// Constraints:
    /// ------------
    /// 1 <= arr.length <= 1000
    /// 0 <= arr[i] <= 1000
    pub fn count_elements(arr: Vec<i32>) -> i32 {
        let mut hmap: HashMap<i32, i32> = HashMap::new();
        let mut count: i32 = 0;

        for x in arr.iter() {
            let item: &mut i32 = hmap.entry(*x).or_insert(0);
            *item += 1;
        }

        for (x, freq) in hmap.iter() {
            if hmap.contains_key(&(*x + 1)) {
                count += freq;
            }
        }

        count
    }

    /// Longest Substring with At Least K Repeating Characters
    ///
    /// https://leetcode.com/problems/longest-substring-with-at-least-k-repeating-characters/
    ///
    /// Given a string s and an integer k, return the length of the longest substring of s such
    /// that the frequency of each character in this substring is greater than or equal to k.
    ///
    /// Example 1:
    /// ----------
    /// Input: s = "aaabb", k = 3
    /// Output: 3
    /// Explanation: The longest substring is "aaa", as 'a' is repeated 3 times.
    ///
    /// Example 2:
    /// ----------
    /// Input: s = "ababbc", k = 2
    /// Output: 5
    /// Explanation: The longest substring is "ababb", as 'a' is repeated 2 times and 'b' is repeated
    /// 3 times.
    ///  
    /// Constraints:
    /// ------------
    /// 1 <= s.length <= 104
    /// s consists of only lowercase English letters.
    /// 1 <= k <= 105
    ///
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let mut counts: HashMap<char, usize> = HashMap::new();
        let mut left = 0;
        let mut ans = 0;
        let s: Vec<char> = s.chars().collect();
        for right in 0..s.len() {
            let x = counts.entry(s[right]).or_insert(0);
            *x += 1;

            while counts.len() as i32 > k {
                let left_char = s[left];
                *counts.get_mut(&left_char).unwrap() -= 1;

                if counts[&left_char] == 0 {
                    counts.remove(&left_char);
                }

                left += 1;
            }

            ans = ans.max((right - left) as i32 + 1);
        }

        ans
    }

    /// ## 2248. Intersection of Multiple Arrays
    /// https://leetcode.com/problems/intersection-of-multiple-arrays/
    /// 
    /// Given a 2D integer array nums where nums[i] is a non-empty array of distinct positive
    /// integers, return the list of integers that are present in each array of nums sorted in
    /// ascending order.
    ///  
    /// Example 1:
    /// ----------
    /// Input: nums = [[3,1,2,4,5],[1,2,3,4],[3,4,5,6]]
    /// Output: [3,4]
    /// Explanation:
    /// The only integers present in each of nums[0] = [3,1,2,4,5], nums[1] = [1,2,3,4], and
    /// nums[2] = [3,4,5,6] are 3 and 4, so we return [3,4].
    ///
    /// Example 2:
    /// ----------
    /// Input: nums = [[1,2,3],[4,5,6]]
    /// Output: []
    /// Explanation:
    /// There does not exist any integer present both in nums[0] and nums[1], so we return an 
    /// empty list [].
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let num_subarr: i32 = nums.len() as i32;
        let mut hashmap: HashMap<i32, i32> = HashMap::new();
        let mut ans: Vec<i32> = vec![];
        // let v: Vec<i32> = (0..10).map(|xx| xx).collect();

        // O(n * m) to populate the hashmap
        for arr in nums {
            for item in arr {
                let x: &mut i32 = hashmap.entry(item).or_insert(0);
                *x += 1;
            }
        }
        // Collect the values
        // Collect the key that satisfies the condition
        for (key, count) in hashmap.iter() {
            if *count == num_subarr {
                ans.push(*key);
            }
        }

        // O(mlogm) for sorting
        ans.sort();
        ans
    }

    pub fn are_occurrences_equal(s: String) -> bool {
        let mut hm: HashMap<char, i32> = HashMap::new();
        let s: Vec<char> = s.chars().collect();
        for key in s.iter() {
            let x: &mut i32 = hm.entry(*key).or_insert(0);
            *x += 1
        }

        let set: HashSet<&i32> = hm.values().collect();
        set.len() == 1
    }

    /// ## 560. Subarray Sum Equals K
    /// https://leetcode.com/problems/subarray-sum-equals-k
    ///
    /// Given an array of integers nums and an integer k, return the total number of 
    /// subarrays whose sum equals to k.
    ///
    /// A subarray is a contiguous non-empty sequence of elements within an array.
    ///
    /// Example 1:
    /// ----------
    /// Input: nums = [1,1,1], k = 2
    /// Output: 2
    /// 
    /// Example 2:
    /// ----------
    /// Input: nums = [1,2,3], k = 3
    /// Output: 2
    ///
    /// Constraints:
    /// -----------
    /// 1 <= nums.length <= 2 * 104
    /// -1000 <= nums[i] <= 1000
    /// -107 <= k <= 107
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans: i32 = 0;
        let mut counts: HashMap<i32, i32> = HashMap::new();
        let mut curr: i32 = 0;
        counts.insert(0, 1);
        for i in nums {
            curr += i;
            // let x: i32 = counts[&(curr - k)];
            ans += *counts.get(&(curr - k)).unwrap_or(&0);
            let prefix_sum_count = counts.entry(curr).or_insert(0);
            *prefix_sum_count += 1;
        }

        ans
    }

    /// ## 2342. Max Sum of a Pair With Equal Sum of Digits
    ///
    /// https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits/
    ///
    /// You are given a 0-indexed array nums consisting of positive integers. You can choose two
    /// indices i and j, such that i != j, and the sum of digits of the number nums[i] is equal
    /// to that of nums[j].
    ///
    /// Return the maximum value of nums[i] + nums[j] that you can obtain over all possible indices
    /// i and j that satisfy the conditions.
    ///
    /// Example 1:
    /// ----------
    /// Input: nums = [18,43,36,13,7]
    /// Output: 54
    /// Explanation: The pairs (i, j) that satisfy the conditions are:
    /// - (0, 2), both numbers have a sum of digits equal to 9, and their sum is 18 + 36 = 54.
    /// - (1, 4), both numbers have a sum of digits equal to 7, and their sum is 43 + 7 = 50.
    /// So the maximum sum that we can obtain is 54.
    ///
    /// Example 2:
    /// ----------
    /// Input: nums = [10,12,19,14]
    /// Output: -1
    /// Explanation: There are no two numbers that satisfy the conditions, so we return -1.
    ///
    /// Constraints:
    /// ----------
    /// 1 <= nums.length <= 105
    /// 1 <= nums[i] <= 109
    ///
    pub fn maximum_sum(_nums: Vec<i32>) -> i32 {
        fn digit_sum(mut n: i32) -> i32 {
            let mut sum: i32 = 0;
            while n > 0 {
                sum += n % 10;
                n /= 10;
            }
            sum
        }

        // Get the digit sum of each number and put it into hashmap
        let mut hm: HashMap<i32, Vec<i32>> = HashMap::new();
        for num in _nums {
            let sum = digit_sum(num);
            let c: &mut Vec<i32> = hm.entry(sum).or_insert(vec![]);
            c.push(num);
        }
        let mut ans = -1;

        for key in hm.clone().keys() {
            let v = hm.get_mut(key).unwrap();
            if v.len() > 1 {
                v.sort_by(|a, b| b.cmp(a));
                ans = ans.max(v[0] + v[1]);
            }
        }
        ans
    }

    /// ## 2260. Minimum Consecutive Cards to Pick Up
    ///
    /// https://leetcode.com/problems/minimum-consecutive-cards-to-pick-up/description/
    ///
    /// You are given an integer array cards where cards[i] represents the value of the ith card.
    /// A pair of cards are matching if the cards have the same value.
    ///
    /// Return the minimum number of consecutive cards you have to pick up to have a pair of matching
    /// cards among the picked cards. If it is impossible to have matching cards, return -1.
    ///
    /// Example 1:
    /// ---------
    /// Input: cards = [3,4,2,3,4,7]
    /// Output: 4
    /// Explanation: We can pick up the cards [3,4,2,3] which contain a matching pair of cards with
    /// value 3. Note that picking up the cards [4,2,3,4] is also optimal.
    ///
    /// Example 2:
    /// ---------
    /// Input: cards = [1,0,5,3]
    /// Output: -1
    /// Explanation: There is no way to pick up a set of consecutive cards that contain a pair of
    /// matching cards.
    ///
    /// Constraints:
    /// -----------
    /// 1 <= cards.length <= 105
    /// 0 <= cards[i] <= 106
    /// 
    /// TODO: Fix tests cases
    pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
        // - Hashmap to store observed card and its indices
        // - Minimum number of cards to pick up between two same cards is the answer
        let mut ans: i32 = i32::MAX;
        let mut hm: HashMap<i32, Vec<i32>> = HashMap::new();
        for (i, c) in cards.iter().enumerate() {
            let v = hm.entry(*c).or_insert(vec![]);
            v.push(i as i32);
        }

        for v in hm.clone().values() {
            for i in 0..(v.len() - 1) {
                ans = ans.min(v[i+1] - v[i] + 1);
            }
        }
        if ans == i32::MAX {-1} else {ans}
    }

    /// ## 2352. Equal Row and Column Pairs
    ///
    /// https://leetcode.com/problems/equal-row-and-column-pairs/
    ///
    /// Given a 0-indexed n x n integer matrix grid, return the number of pairs (ri, cj) such that
    /// row ri and column cj are equal.
    ///
    /// A row and column pair is considered equal if they contain the same elements in the same
    /// order (i.e., an equal array).
    ///
    /// Example 1:
    /// ----------
    /// Input: grid = [[3,2,1],[1,7,6],[2,7,7]]
    /// Output: 1
    /// Explanation: There is 1 equal row and column pair:
    /// - (Row 2, Column 1): [2,7,7]
    ///
    /// Example 2:
    /// ----------
    /// Input: grid = [[3,1,2,2],[1,4,4,5],[2,4,2,2],[2,4,2,2]]
    /// Output: 3
    /// Explanation: There are 3 equal row and column pairs:
    /// - (Row 0, Column 0): [3,1,2,2]
    /// - (Row 2, Column 2): [2,4,2,2]
    /// - (Row 3, Column 2): [2,4,2,2]
    ///
    /// Constraints:
    /// -------------
    /// n == grid.length == grid[i].length
    /// 1 <= n <= 200
    /// 1 <= grid[i][j] <= 105
    /// 
    /// Algorithm
    /// - Iterate column wise and create tuples of item as key
    /// and save it into hashmap with value as counter.
    /// - Iterate row wise and do the same as above
    /// - Iterate over row and col hashmap and multiply the row value 
    /// with the corresponding element in col hashmap.
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut rows_map: HashMap<Vec<i32>, i32> = HashMap::new();
        let mut cols_map: HashMap<Vec<i32>, i32> = HashMap::new();
        let mut counter: i32 = 0;
        let (rows, cols) = (grid.len(), grid[0].len());
        for i in 0..rows {
            let key = grid[i].clone();
            let entry: &mut i32 = rows_map.entry(key).or_insert(0);
            *entry += 1;
        }

        for i in 0..cols {
            let mut key: Vec<i32> = vec![];
            for j in 0..rows {
                key.push(grid[j][i].clone());
            }
            let entry: &mut i32 = cols_map.entry(key).or_insert(0);
            *entry += 1;
        }
        // Iterate the rows keys and multiply it's value with the corresponding 
        // value in cols
        for (key, value) in rows_map {
            let c: &i32 = cols_map.get(&key).unwrap_or(&0);
            counter += value * (*c);
        }
        
        counter
    }


    /// ## 1248. Count Number of Nice Subarrays
    ///
    /// https://leetcode.com/problems/count-number-of-nice-subarrays/description/
    /// Given an array of integers nums and an integer k. A continuous subarray is called nice if
    /// there are k odd numbers on it.
    ///
    /// Return the number of nice sub-arrays.
    ///
    ///
    /// Example 1:
    /// ----------
    /// Input: nums = [1,1,2,1,1], k = 3
    /// Output: 2
    /// Explanation: The only sub-arrays with 3 odd numbers are [1,1,2,1] and [1,2,1,1].
    ///
    /// Example 2:
    /// ----------
    /// Input: nums = [2,4,6], k = 1
    /// Output: 0
    /// Explanation: There is no odd numbers in the array.
    /// 
    /// Example 3:
    /// ----------
    /// Input: nums = [2,2,2,1,2,2,1,2,2,2], k = 2
    /// Output: 16
    ///
    /// Constraints:
    /// ----------  
    /// 1 <= nums.length <= 50000
    /// 1 <= nums[i] <= 10^5
    /// 1 <= k <= nums.length
    ///
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        // Count the number of odd numbers in the array
        let mut ans: i32 = 0;
        let mut curr_count: i32 = 0;
        // HashMap to store the count of odd numbers until the current index
        let mut hm: HashMap<i32, i32> = HashMap::new();
        hm.insert(0, 1);
        
        for num in nums {
            // Count the number of odd numbers
            curr_count += num % 2;
            ans += hm.get(&(curr_count - k)).unwrap_or(&0);
            let entry = hm.entry(curr_count).or_insert(0);
            *entry += 1;
        }
        ans
    }

    /// ## 1832. Check if the Sentence Is Pangram
    ///
    /// A pangram is a sentence where every letter of the English alphabet appears at least once.
    ///
    /// Given a string sentence containing only lowercase English letters, return true if sentence 
    /// is a pangram, or false otherwise.
    ///
    /// Example 1:
    /// ----------
    /// Input: sentence = "thequickbrownfoxjumpsoverthelazydog"
    /// Output: true
    /// Explanation: sentence contains at least one of every letter of the English alphabet.
    /// 
    /// Example 2:
    /// ----------
    /// Input: sentence = "leetcode"
    /// Output: false
    ///
    /// Constraints:
    /// ----------
    /// 1 <= sentence.length <= 1000
    /// sentence consists of lowercase English letters.
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut hm: HashMap<char, i32> = HashMap::new();
        for c in sentence.chars() {
            let entry = hm.entry(c).or_insert(0);
            *entry += 1;
        }

        hm.keys().len() == 26
    }

    /// ## 2225. Find Players With Zero or One Losses
    ///
    /// You are given an integer array matches where matches[i] = [winneri, loseri] indicates 
    /// that the player winneri defeated player loseri in a match.
    ///
    /// Return a list answer of size 2 where:
    /// - answer[0] is a list of all players that have not lost any matches.
    /// - answer[1] is a list of all players that have lost exactly one match.
    ///
    /// The values in the two lists should be returned in increasing order.
    ///
    /// Note:
    ///
    /// You should only consider the players that have played at least one match.
    /// The testcases will be generated such that no two matches will have the same outcome.
    ///
    ///
    /// Example 1:
    /// ----------
    /// Input: matches = [[1,3],[2,3],[3,6],[5,6],[5,7],[4,5],[4,8],[4,9],[10,4],[10,9]]
    /// Output: [[1,2,10],[4,5,7,8]]
    /// Explanation:
    /// Players 1, 2, and 10 have not lost any matches.
    /// Players 4, 5, 7, and 8 each have lost one match.
    /// Players 3, 6, and 9 each have lost two matches.
    /// Thus, answer[0] = [1,2,10] and answer[1] = [4,5,7,8].
    /// 
    /// Example 2:
    /// ----------
    /// Input: matches = [[2,3],[1,3],[5,4],[6,4]]
    /// Output: [[1,2,5,6],[]]
    /// Explanation:
    /// Players 1, 2, 5, and 6 have not lost any matches.
    /// Players 3 and 4 each have lost two matches.
    /// Thus, answer[0] = [1,2,5,6] and answer[1] = [].
    ///
    /// Algorithm: 
    /// - Use HashSet to track the winner and loser entry, this will be useful to find user 
    /// who have not lost any match i.e. diff of winner - loser
    /// - Track the count of loser and return those entry with only one lost
    /// 
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut winners_sets: HashSet<i32> = HashSet::new();
        let mut losers_sets: HashSet<i32> = HashSet::new();
        let mut loser_count: HashMap<i32, i32> = HashMap::new();

        for match_ in matches {
            let (winner, loser) = (match_[0], match_[1]);
            winners_sets.insert(winner);
            losers_sets.insert(loser);

            // Increment the count of loser
            let entry = loser_count.entry(loser).or_insert(0);
            *entry += 1;
        }

        let mut x = winners_sets
            .difference(&losers_sets)
            .map(|x| *x).collect::<Vec<i32>>();
        x.sort();
        
        ans.append(&mut vec![x]);

        let mut y = 
            loser_count.into_iter()
            .filter(|(_, v)| *v == 1)
            .map(|(k, _)| k).collect::<Vec<i32>>();
        y.sort();
        
        ans.append(&mut vec![y]);
        
        ans 
    }

    /// ## 1133. Largest Unique Number
    ///
    /// Given an integer array nums, return the largest integer that only occurs once. 
    /// If no integer occurs once, return -1.
    ///
    /// Example 1:
    /// ----------
    /// Input: nums = [5,7,3,9,4,9,8,3,1]
    /// Output: 8
    /// Explanation: The maximum integer in the array is 9 but it is repeated. The number 
    /// 8 occurs only once, so it is the answer.
    /// 
    /// Example 2:
    /// ----------
    /// Input: nums = [9,9,8,8]
    /// Output: -1
    /// Explanation: There is no number that occurs only once.
    ///
    ///
    /// Constraints:
    /// ------------
    /// 1 <= nums.length <= 2000
    /// 0 <= nums[i] <= 1000
    /// 
    /// Algorithm:
    /// - Use HashMap to track the count of each number
    /// - Sort by key in descending order
    /// - Return the largest key if count is 1
    pub fn largest_unique_number(nums: Vec<i32>) -> i32 {
        let mut hm: HashMap<i32, i32> = HashMap::new();

        for num in nums {
            let entry = hm.entry(num).or_insert(0);
            *entry += 1;
        }

        let ans = hm.into_iter()
            .filter(|(_, v)| *v == 1)
            .map(|(k, _)| k).max();
        
        if let Some(ans) = ans {
            return ans
        }

        -1
    }

    /// ## 1189. Maximum Number of Balloons
    /// 
    /// Given a string text, you want to use the characters of text to form as many instances 
    /// of the word "balloon" as possible.
    ///
    /// You can use each character in text at most once. Return the maximum number of instances 
    /// that can be formed.
    ///
    /// Example 1:
    /// ----------
    /// Input: text = "nlaebolko"
    /// Output: 1
    ///
    /// Example 2:
    /// ----------
    /// Input: text = "loonbalxballpoon"
    /// Output: 2
    ///
    /// Example 3:
    /// ----------
    /// Input: text = "leetcode"
    /// Output: 0
    ///
    ///
    /// Constraints:
    ///
    /// 1 <= text.length <= 104
    /// text consists of lower case English letters only.
    /// 
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut hm: HashMap<char, i32> = HashMap::new();
        
        for c in text.chars() {
            *hm.entry(c).or_insert(0) += 1;
        }
        let mut ans: i32 = 0;
        loop {
            for c in "balloon".chars() {
                if hm.contains_key(&c) && *hm.get(&c).unwrap() > 0 {
                    *hm.get_mut(&c).unwrap() -= 1;
                } else {
                    return ans;
                }
            }
            ans += 1;
        }
    }

    /// ## 383. Ransom Note
    ///
    /// Given two strings ransomNote and magazine, return true if ransomNote can be constructed 
    /// by using the letters from magazine and false otherwise.
    ///
    /// Each letter in magazine can only be used once in ransomNote.
    ///
    /// Example 1:
    /// ----------
    /// Input: ransomNote = "a", magazine = "b"
    /// Output: false
    /// 
    /// Example 2:
    /// ----------
    /// Input: ransomNote = "aa", magazine = "ab"
    /// Output: false
    ///
    /// Example 3:
    ///
    /// Input: ransomNote = "aa", magazine = "aab"
    /// Output: true
    ///
    ///
    /// Constraints:
    /// ------------
    /// 1 <= ransomNote.length, magazine.length <= 105
    /// ransomNote and magazine consist of lowercase English letters.
    /// 
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut hm: HashMap<char, i32> = HashMap::new();
        for c in ransom_note.chars() {
            *hm.entry(c).or_insert(0) += 1;
        }

        for c in magazine.chars() {
            if hm.contains_key(&c) && *hm.get(&c).unwrap() > 0 {
                *hm.get_mut(&c).unwrap() -= 1;

                if *hm.get(&c).unwrap() == 0 {
                    hm.remove(&c);
                }
            }
        }

        hm.is_empty()
    }


    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let hs: HashSet<char> = jewels.chars().collect();

        let mut hm: HashMap<char, i32> = HashMap::new();
        for c in stones.chars() {
            let entry = hm.entry(c).or_insert(0);
            if hs.contains(&c) {
                *entry += 1;
            }
        }
        
        hm.values().into_iter().sum()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_number_of_balloons() {
        let res = Solution::max_number_of_balloons("nlaebolko".to_string());
        assert_eq!(res, 1);

        let res = Solution::max_number_of_balloons("loonbalxballpoon".to_string());
        assert_eq!(res, 2);

        let res = Solution::max_number_of_balloons("leetcode".to_string());
        assert_eq!(res, 0);
    }

    #[test]
    fn test_minimum_card_pickup() {
        let cards = vec![3,4,2,3,4,7];
        assert_eq!(Solution::minimum_card_pickup(cards), 4);

        let cards = vec![1,0,5,3];
        assert_eq!(Solution::minimum_card_pickup(cards), -1);
    }

    #[test]
    fn test_longest_substring() {
        let s = "aaabb".to_string();
        let k = 3;
        assert_eq!(Solution::longest_substring(s, k), 3);

        let s = "ababbc".to_string();
        let k = 2;
        assert_eq!(Solution::longest_substring(s, k), 5);
    }

    #[test]
    fn test_equal_pairs() {
        let grid = vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]];
        assert_eq!(Solution::equal_pairs(grid), 1);

        let grid = vec![vec![3, 1, 2, 2], vec![1, 4, 4, 5], vec![2, 4, 2, 2], vec![2, 4, 2, 2]];
        assert_eq!(Solution::equal_pairs(grid), 3);
    }

    #[test]
    pub fn test_number_of_subarrays() {
        let nums = vec![1, 1, 2, 1, 1];
        let k = 3;
        assert_eq!(Solution::number_of_subarrays(nums, k), 2);

        let nums = vec![2, 4, 6];
        let k = 1;
        assert_eq!(Solution::number_of_subarrays(nums, k), 0);

        let nums: Vec<i32> = vec![2,2,2,1,2,2,1,2,2,2];
        let k = 2;
        assert_eq!(Solution::number_of_subarrays(nums, k), 16);
    }
}

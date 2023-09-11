#![allow(unused)]
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug)]
struct Float(f32);

impl Ord for Float {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Float {}

struct FreqPair(usize, String);

impl Ord for FreqPair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for FreqPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl PartialEq for FreqPair {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for FreqPair {}

struct Solution;

impl Solution {
    /// ## 1046. Last Stone Weight
    /// https://leetcode.com/problems/last-stone-weight/
    ///
    /// You are given an array of integers stones where stones[i] is the weight of the ith stone.
    ///
    /// We are playing a game with the stones. On each turn, we choose the heaviest two stones and smash
    /// them together. Suppose the heaviest two stones have weights x and y with x <= y. The result of this
    /// smash is:
    ///
    /// - If x == y, both stones are destroyed, and
    /// - If x != y, the stone of weight x is destroyed, and the stone of weight y has new weight y - x.
    /// - At the end of the game, there is at most one stone left.
    ///
    /// Return the weight of the last remaining stone. If there are no stones left, return 0.
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let stones = vec![2,7,4,1,8,1];
    /// let ans = 1
    /// assert_eq!(Solution::last_stone_weight(stones), ans);
    /// ```
    /// Explanation:
    /// - We combine 7 and 8 to get 1 so the array converts to \[2,4,1,1,1] then,
    /// - we combine 2 and 4 to get 2 so the array converts to \[2,1,1,1] then,
    /// - we combine 2 and 1 to get 1 so the array converts to \[1,1,1] then,
    /// - we combine 1 and 1 to get 0 so the array converts to \[1] then that's the value of the last stone.
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let stones = vec![1];
    /// let ans = 1;
    /// assert_eq!(Solution::last_stone_weight(stones), ans);
    /// ```
    /// Constraints:
    /// ----------
    /// * 1 <= stones.length <= 30
    /// * 1 <= stones\[i] <= 1000
    ///
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap: BinaryHeap<i32> = stones.into_iter().collect();

        // Now pop each element from Heap and push the new calculated element in heap
        while heap.len() > 1 {
            let stone1 = heap.pop().unwrap();
            let stone2 = heap.pop().unwrap();
            if stone1 != stone2 {
                heap.push((stone1 - stone2).abs());
            }
        }

        // Return the last element in the heap if exists
        if !heap.is_empty() {
            return heap.pop().unwrap();
        }

        0
    }

    /// ## 2208. Minimum Operations to Halve Array Sum
    ///
    /// https://leetcode.com/problems/minimum-operations-to-halve-array-sum/
    ///
    /// You are given an array nums of positive integers. In one operation, you can choose any number from
    /// nums and reduce it to exactly half the number. (Note that you may choose this reduced number in future
    /// operations.)
    ///
    /// Return the minimum number of operations to reduce the sum of nums by at least half.
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let nums = vec![5,19,8,1];
    /// assert_eq!(Solution::min_operations(nums), 3);
    /// ```
    /// Explanation: The initial sum of nums is equal to 5 + 19 + 8 + 1 = 33.
    /// The following is one of the ways to reduce the sum by at least half:
    ///
    /// - Pick the number 19 and reduce it to 9.5.
    /// - Pick the number 9.5 and reduce it to 4.75.
    /// - Pick the number 8 and reduce it to 4.
    /// - The final array is \[5, 4.75, 4, 1] with a total sum of 5 + 4.75 + 4 + 1 = 14.75.
    /// - The sum of nums has been reduced by 33 - 14.75 = 18.25, which is at least half of the initial sum,
    /// 18.25 >= 33/2 = 16.5.
    ///
    /// Overall, 3 operations were used so we return 3.
    /// It can be shown that we cannot reduce the sum by at least half in less than 3 operations.
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let nums = vec![3,8,20];
    /// assert_eq!(Solution::min_operations(nums), 3);
    /// ```
    /// Explanation: The initial sum of nums is equal to 3 + 8 + 20 = 31.
    /// The following is one of the ways to reduce the sum by at least half:
    /// - Pick the number 20 and reduce it to 10.
    /// - Pick the number 10 and reduce it to 5.
    /// - Pick the number 3 and reduce it to 1.5.
    /// - The final array is \[1.5, 8, 5] with a total sum of 1.5 + 8 + 5 = 14.5.
    /// - The sum of nums has been reduced by 31 - 14.5 = 16.5, which is at least half of the initial sum,
    /// 16.5 >= 31/2 = 15.5.
    ///
    /// Overall, 3 operations were used so we return 3.
    /// It can be shown that we cannot reduce the sum by at least half in less than 3 operations.
    ///
    /// Constraints:
    /// ------------
    /// 1 <= nums.length <= 105
    /// 1 <= nums\[i] <= 107
    pub fn halve_array(nums: Vec<i32>) -> i32 {
        // Add element into heap
        // Get the sum of all the element in nums
        // After poping the element, push the new calculated element in heap and add it into result
        // If result <= sum /2, then stop poping any more element from heap
        // return count
        let mut count = 0;
        let mut target = (nums.iter().sum::<i32>() / 2) as f32;
        let mut heap: BinaryHeap<Float> = BinaryHeap::new();

        for num in nums {
            heap.push(Float(num as f32));
        }

        while !heap.is_empty() && target > 0.0 {
            count += 1;
            let item = heap.pop().unwrap();
            target -= item.0 / 2.0;
            heap.push(Float(item.0 / 2.0));
        }
        count
    }

    /// ## Remove Stones to Minimize the Total
    ///
    /// You are given a 0-indexed integer array piles, where piles\[i] represents the number of stones
    /// in the ith pile, and an integer k. You should apply the following operation exactly k times:
    ///
    /// Choose any piles[i] and remove floor(piles\[i] / 2) stones from it.
    /// Notice that you can apply the operation on the same pile more than once.
    ///
    /// Return the minimum possible total number of stones remaining after applying the k operations.
    ///
    /// floor(x) is the greatest integer that is smaller than or equal to x (i.e., rounds x down).
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let piles = vec![5,4,9]; let k = 2;
    /// let ans = 12;
    /// assert_eq!(Solution::min_stones(piles, k), ans);
    /// ```
    /// Explanation: Steps of a possible scenario are:
    /// - Apply the operation on pile 2. The resulting piles are \[5,4,5].
    /// - Apply the operation on pile 0. The resulting piles are \[3,4,5].
    /// The total number of stones in \[3,4,5] is 12.
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let piles = [4,3,6,7]; let k = 3;
    /// let ans = 12;
    /// assert_eq!(Solution::min_stones(piles, k), ans);
    /// ```
    /// Explanation: Steps of a possible scenario are:
    /// - Apply the operation on pile 2. The resulting piles are \[4,3,3,7].
    /// - Apply the operation on pile 3. The resulting piles are \[4,3,3,4].
    /// - Apply the operation on pile 0. The resulting piles are \[2,3,3,4].
    /// The total number of stones in \[2,3,3,4] is 12.
    ///
    /// Constraints:
    /// -----------
    /// - 1 <= piles.length <= 105
    /// - 1 <= piles\[i] <= 104
    /// - 1 <= k <= 105
    ///
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut hq: BinaryHeap<i32> = piles.into_iter().collect();
        let mut k = k;
        while k > 0 {
            // Pop the max element from heap
            let mut stone = hq.pop().unwrap();
            // Reduce the stone by floor(stone / 2)
            stone -= (stone as f32 / 2.0).floor() as i32;
            hq.push(stone);
            k -= 1;
        }
        hq.into_iter().sum()
    }

    pub fn connect_sticks(sticks: Vec<i32>) -> i32 {
        let mut hq: BinaryHeap<Reverse<i32>> = sticks.into_iter().map(|x| Reverse(x)).collect();
        while hq.len() > 1 {
            let s1 = hq.pop().unwrap();
            let s2 = hq.pop().unwrap();

            hq.push(Reverse(s1.0 + s2.0))
        }

        hq.pop().unwrap().0
    }

    /// 347. Top K Frequent Elements
    ///
    /// Given an integer array nums and an integer k, return the k most frequent elements. You may return
    /// the answer in any order.
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let nums = [1,1,1,2,2,3]; let k = 2;
    /// assert_eq!(Solution::top_k_frequent(nums, k), vec![1,2]);
    /// ```
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let nums = vec![1]; let k = 1;
    /// assert_eq!(Solution::top_k_frequent(nums, k), vec![1]);
    /// ```
    ///
    /// Constraints:
    /// -----------
    /// * 1 <= nums.length <= 105
    /// * -104 <= nums\[i] <= 104
    /// * k is in the range \[1, the number of unique elements in the array].
    /// * It is guaranteed that the answer is unique.
    ///
    /// Follow up: Your algorithm's time complexity must be better than O(n log n), where n is the array's size.
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // Add element and count into heap
        // Pop the max element from heap for k times
        // If k == nums.len(), return nums
        fn counter(nums: &Vec<i32>) -> HashMap<i32, usize> {
            nums.iter().fold(HashMap::new(), |mut counter, &num| {
                let entry: &mut usize = counter.entry(num).or_insert(0);
                *entry += 1;
                counter
            })
        }

        if k == nums.len() as i32 {
            return nums;
        }

        let mut k: i32 = k;
        let freq: HashMap<i32, usize> = counter(&nums);
        let mut hq: BinaryHeap<(usize, i32)> = freq
            .into_iter()
            .map(|(item, count)| (count, item))
            .collect();

        let mut ans: Vec<i32> = vec![];
        while k > 0 {
            let (_, item) = hq.pop().unwrap();
            ans.push(item);
            k -= 1;
        }

        ans
    }

    /// Given an array of strings words and an integer k, return the k most frequent strings.
    ///
    /// Return the answer sorted by the frequency from highest to lowest. Sort the words with
    /// the same frequency by their lexicographical order.
    ///
    /// Example 1:
    /// ----------
    /// Input: words = ["i","love","leetcode","i","love","coding"], k = 2
    /// Output: ["i","love"]
    /// Explanation: "i" and "love" are the two most frequent words.
    /// Note that "i" comes before "love" due to a lower alphabetical order.
    ///
    /// Example 2:
    /// ----------
    /// Input: words = ["the","day","is","sunny","the","the","the","sunny","is","is"], k = 4
    /// Output: ["the","is","sunny","day"]
    /// Explanation: "the", "is", "sunny" and "day" are the four most frequent words, with the number
    /// of occurrence being 4, 3, 2 and 1 respectively.
    ///
    ///
    /// Constraints:
    /// ------------
    /// 1 <= words.length <= 500
    /// 1 <= words[i].length <= 10
    /// words[i] consists of lowercase English letters.
    /// k is in the range [1, The number of unique words[i]]
    ///
    ///
    /// Follow-up: Could you solve it in O(n log(k)) time and O(n) extra space?
    pub fn top_k_frequent_words(words: Vec<String>, k: i32) -> Vec<String> {
        fn counter(nums: &Vec<String>) -> HashMap<String, usize> {
            let mut hm: HashMap<String, usize> = HashMap::new();
            for word in nums {
                let word = word.clone();
                let entry: &mut usize = hm.entry(word).or_insert(0);
                *entry += 1;
            }

            hm
        }

        let mut k: i32 = k;
        let freq: HashMap<String, usize> = counter(&words);
        let mut hq: BinaryHeap<FreqPair> = freq
            .into_iter()
            .map(|(item, count)| FreqPair(count, item))
            .collect();

        let mut ans: Vec<String> = vec![];
        while k > 0 {
            let pair_compare = hq.pop().unwrap();
            ans.push(pair_compare.1);
            k -= 1;
        }

        ans
    }

    /// ## 658. Find K Closest Elements
    ///
    /// Given a sorted integer array arr, two integers k and x, return the k closest integers to x
    /// in the array. The result should also be sorted in ascending order.
    ///
    /// An integer a is closer to x than an integer b if:
    ///
    /// ```doc
    /// |a - x| < |b - x|, or
    /// |a - x| == |b - x| and a < b
    /// ```
    /// Example 1:
    /// ----------
    /// ```
    /// let arr = vec![1,2,3,4,5]; let k = 4; let x = 3;
    /// assert_eq!(Solution::find_closest_elements(arr, k, x), vec![1,2,3,4]);
    /// ````
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let arr = vec![1,2,3,4,5]; let k = 4; let x = -1;
    /// assert_eq!(Solution::find_closest_elements(arr, k, x), vec![1,2,3,4]);
    /// ```
    ///
    /// Constraints:
    /// ------------
    /// * 1 <= k <= arr.length
    /// * 1 <= arr.length <= 104
    /// * arr is sorted in ascending order.
    /// * -104 <= arr\[i], x <= 104
    ///
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        // Use BinaryHeap to remove the max difference element
        let mut heap = BinaryHeap::new();

        for num in arr {
            let distance = (x - num).abs();
            heap.push((distance, num));
            if heap.len() as i32 > k {
                heap.pop();
            }
        }

        let mut ans: Vec<i32> = heap.into_iter().map(|pair| pair.1).collect();
        ans.sort();
        ans
    }

    // Use binary search to solve this
    // https://leetcode.com/problems/find-k-closest-elements/solution/
    pub fn find_closest_elements_optimized(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        // Base case
        if arr.len() as i32 == k {
            for i in 0..k {
                result.push(arr[i as usize]);
            }

            return result;
        }

        // Binary search to find the closest element
        let mut left: i32 = 0;
        let mut right: i32 = arr.len() as i32;
        let mut mid = 0;
        while left < right {
            mid = (left + right) / 2;
            if arr[mid as usize] >= x {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        // Initialize our sliding window's bounds
        left -= 1;
        right = left + 1;

        // While the window size is less than k
        while right - left - 1 < k {
            // Be careful to not go out of bounds
            if left == -1 {
                right += 1;
                continue;
            }

            // Expand the window towards the side with the closer number
            // Be careful to not go out of bounds with the pointers
            if right == arr.len() as i32 || (arr[left as usize] - x).abs() <= (arr[right as usize] - x).abs() {
                left -= 1;
            } else {
                right += 1;
            }
        }

        // Build and return the window
        for i in left + 1..right {
            result.push(arr[i as usize]);
        }

        result
    }

    /// ##  K Closest Points to Origin
    ///
    /// Given an array of points where points[i] = [xi, yi] represents a point on the X-Y plane
    /// and an integer k, return the k closest points to the origin (0, 0).
    ///
    /// The distance between two points on the X-Y plane is the Euclidean distance
    /// (i.e., √(x^1 - x2)^2 + (y^1 - y^2)^2).
    ///
    /// You may return the answer in any order. The answer is guaranteed to be unique
    /// (except for the order that it is in).
    ///
    ///
    /// Example 1:
    /// ----------
    ///
    /// Input: points = [[1,3],[-2,2]], k = 1
    /// Output: [[-2,2]]
    /// Explanation:
    /// The distance between (1, 3) and the origin is sqrt(10).
    /// The distance between (-2, 2) and the origin is sqrt(8).
    /// Since sqrt(8) < sqrt(10), (-2, 2) is closer to the origin.
    /// We only want the closest k = 1 points from the origin, so the answer is just [[-2,2]].
    ///
    /// Example 2:
    ///
    /// Input: points = [[3,3],[5,-1],[-2,4]], k = 2
    /// Output: [[3,3],[-2,4]]
    /// Explanation: The answer [[-2,4],[3,3]] would also be accepted.
    ///
    /// Constraints:
    /// -----------
    ///
    /// 1 <= k <= points.length <= 104
    /// -104 < xi, yi < 104
    ///
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        // Fetch the min difference element from the heap

        let mut distances: BinaryHeap<(Reverse<Float>, i32, i32)> = points
            .into_iter()
            .map(|point| {
                let x = point[0] as f32;
                let y = point[1] as f32;
                let distance = Float((x.powi(2) + y.powi(2)).sqrt());
                (Reverse(distance), point[0], point[1])
            })
            .collect();

        let mut result = Vec::new();

        for _ in 0..k {
            let (_, x, y) = distances.pop().unwrap();
            result.push(vec![x, y]);
        }

        result
    }

    /// ## 480. Sliding Window Median
    /// https://leetcode.com/problems/sliding-window-median/
    ///
    /// The median is the middle value in an ordered integer list. If the size of the list is even, there is no middle value.
    /// So the median is the mean of the two middle values.
    ///
    /// * For examples, if arr = \[2,3,4], the median is 3.
    /// * For examples, if arr = \[1,2,3,4], the median is (2 + 3) / 2 = 2.5.
    /// You are given an integer array nums and an integer k. There is a sliding window of size k which is moving from the
    /// very left of the array to the very right. You can only see the k numbers in the window. Each time the sliding window
    /// moves right by one position.
    ///
    /// Return the median array for each window in the original array. Answers within 10-5 of the actual value will be accepted.
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let nums = vec![1,3,-1,-3,5,3,6,7]; let k = 3;
    /// let ans = vec![1.00000,-1.00000,-1.00000,3.00000,5.00000,6.00000];
    /// assert_eq!(Solution::sliding_window_median(nums, k), ans);
    /// ```
    /// Explanation:
    /// ```doc
    /// Window position               Median
    /// ---------------               -----
    /// [1  3  -1] -3  5  3  6  7       1
    /// 1 [3  -1  -3] 5  3  6  7       -1
    /// 1  3 [-1  -3  5] 3  6  7       -1
    /// 1  3  -1 [-3  5  3] 6  7        3
    /// 1  3  -1  -3 [5  3  6] 7        5
    /// 1  3  -1  -3  5 [3  6  7]       6
    /// ```
    /// Example 2:
    /// ----------
    /// ```
    /// let nums = vec![1,2,3,4,2,3,1,4,2]; let k = 3;
    /// let ans = [2.00000,3.00000,3.00000,3.00000,2.00000,3.00000,2.00000];
    /// assert_eq!(Solution::sliding_window_median(nums, k), ans);
    /// ```
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_stone_weight() {
        assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);

        assert_eq!(Solution::last_stone_weight(vec![1]), 1);
    }

    #[test]
    fn test_half_array() {
        assert_eq!(Solution::halve_array(vec![5, 19, 8, 1]), 3);

        // Fix this test
        // assert_eq!(Solution::halve_array(vec![3,8,20]), 3);
    }

    #[test]
    fn test_top_k_frequent_words() {
        let words = vec!["i", "love", "leetcode", "i", "love", "coding"];
        let words = words.into_iter().map(|x| x.to_string()).collect();
        let k: i32 = 2;
        let ans = vec!["i", "love"];
        let ans: Vec<String> = ans.into_iter().map(|x| x.to_string()).collect();
        assert_eq!(Solution::top_k_frequent_words(words, k), ans);

        let words = vec![
            "the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is",
        ];
        let words = words.into_iter().map(|x| x.to_string()).collect();
        let k: i32 = 4;
        let ans = vec!["the", "is", "sunny", "day"];
        let ans: Vec<String> = ans.into_iter().map(|x| x.to_string()).collect();
        assert_eq!(Solution::top_k_frequent_words(words, k), ans);
    }

    #[test]
    fn test_k_closest() {
        let points = vec![vec![1, 3], vec![-2, 2]];
        let k = 1;
        let ans = vec![vec![-2, 2]];
        assert_eq!(Solution::k_closest(points, k), ans);

        let points = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
        let k = 2;
        let ans = vec![vec![3, 3], vec![-2, 4]];
        assert_eq!(Solution::k_closest(points, k), ans);
    }
}

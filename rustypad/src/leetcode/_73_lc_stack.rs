#![allow(unused)]

use std::collections::{HashMap, VecDeque};


/// ## 933. Number of Recent Calls
///
/// You have a RecentCounter class which counts the number of recent requests within a certain time frame.
///
/// Implement the RecentCounter class:
///
/// RecentCounter() Initializes the counter with zero recent requests.
/// int ping(int t) Adds a new request at time t, where t represents some time in milliseconds, and returns 
/// the number of requests that has happened in the past 3000 milliseconds (including the new request). 
/// Specifically, return the number of requests that have happened in the inclusive range [t - 3000, t].
/// It is guaranteed that every call to ping uses a strictly larger value of t than the previous call.
///  
///
/// Example 1:
/// ----------
/// Input
/// ["RecentCounter", "ping", "ping", "ping", "ping"]
/// [[], [1], [100], [3001], [3002]]
/// Output
/// [null, 1, 2, 3, 3]
///
/// Explanation
/// RecentCounter recentCounter = new RecentCounter();
/// recentCounter.ping(1);     // requests = [1], range is [-2999,1], return 1
/// recentCounter.ping(100);   // requests = [1, 100], range is [-2900,100], return 2
/// recentCounter.ping(3001);  // requests = [1, 100, 3001], range is [1,3001], return 3
/// recentCounter.ping(3002);  // requests = [1, 100, 3001, 3002], range is [2,3002], return 3
///  
///
/// Constraints:
/// -----------
/// 1 <= t <= 109
/// Each test case will call ping with strictly increasing values of t.
/// At most 104 calls will be made to ping.
/// 
/// Create a vecdeque to store the input time in order
struct RecentCounter {
    q: VecDeque<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        RecentCounter { q: VecDeque::new() }
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        while self.q.len() > 0 && t - *self.q.front().unwrap() > 3000  {
            self.q.pop_front();
        }
        self.q.push_back(t);
        self.q.len() as i32
    }
}

/// ## Moving Average from Data Stream
///
/// Given a stream of integers and a window size, calculate the moving average of all integers in the sliding window.
///
/// Implement the MovingAverage class:
///
/// MovingAverage(int size) Initializes the object with the size of the window size.
/// double next(int val) Returns the moving average of the last size values of the stream.
///  
/// Example 1:
/// ----------
/// Input
/// ["MovingAverage", "next", "next", "next", "next"]
/// [[3], [1], [10], [3], [5]]
/// Output
/// [null, 1.0, 5.5, 4.66667, 6.0]
///
/// Explanation
/// MovingAverage movingAverage = new MovingAverage(3);
/// movingAverage.next(1); // return 1.0 = 1 / 1
/// movingAverage.next(10); // return 5.5 = (1 + 10) / 2
/// movingAverage.next(3); // return 4.66667 = (1 + 10 + 3) / 3
/// movingAverage.next(5); // return 6.0 = (10 + 3 + 5) / 3
///  
///
/// Constraints:
/// -----------
/// 1 <= size <= 1000
/// -105 <= val <= 105
/// At most 104 calls will be made to next.
/// 
struct MovingAverage {
    dq: VecDeque<i32>,
    sum: i32,
    size: i32
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {

    fn new(size: i32) -> Self {
        MovingAverage {
            dq: VecDeque::new(),
            sum: 0,
            size
        }
    }
    
    fn next(&mut self, val: i32) -> f64 {
        if self.dq.len() > self.size as usize {
            self.sum -= self.dq.pop_front().unwrap();
        }
        self.dq.push_back(val);
        self.sum += val;
        self.sum as f64 / self.dq.len() as f64
    }
}

/**
 * Your MovingAverage object will be instantiated and called as such:
 * let obj = MovingAverage::new(size);
 * let ret_1: f64 = obj.next(val);
 */


struct Solution;

impl Solution {
    

    /// ## 71. Simplify Path
    ///
    /// Given a string path, which is an absolute path (starting with a slash '/') to a file or directory 
    /// in a Unix-style file system, convert it to the simplified canonical path.
    ///
    /// In a Unix-style file system, a period '.' refers to the current directory, a double period '..' refers 
    /// to the directory up a level, and any multiple consecutive slashes (i.e. '//') are treated as a single 
    /// slash '/'. For this problem, any other format of periods such as '...' are treated as file/directory names.
    ///
    /// The canonical path should have the following format:
    ///
    /// The path starts with a single slash '/'.
    /// Any two directories are separated by a single slash '/'.
    /// The path does not end with a trailing '/'.
    /// The path only contains the directories on the path from the root directory to the target file or directory 
    /// (i.e., no period '.' or double period '..')
    /// Return the simplified canonical path.
    ///
    /// Example 1:
    /// ----------
    /// Input: path = "/home/"
    /// Output: "/home"
    /// Explanation: Note that there is no trailing slash after the last directory name.
    ///
    /// Example 2:
    /// ----------
    /// Input: path = "/../"
    /// Output: "/"
    /// Explanation: Going one level up from the root directory is a no-op, as the root level is the highest level 
    /// you can go.
    ///
    /// Example 3:
    /// ----------
    /// Input: path = "/home//foo/"
    /// Output: "/home/foo"
    /// Explanation: In the canonical path, multiple consecutive slashes are replaced by a single one.
    ///
    ///
    /// Constraints:
    /// ------------
    /// 1 <= path.length <= 3000
    /// path consists of English letters, digits, period '.', slash '/' or '_'.
    /// path is a valid absolute Unix path.
    /// 
    pub fn simplify_path(path: String) -> String {
        // Split the path into a vec of strings delimited by '/'
        // Add into stack if it is a directory and pop if it is ..
        let paths: Vec<&str> = path.split('/').collect::<Vec<&str>>();
        let mut stack: Vec<String> = vec![];

        for section in paths {
            if section == "." || section.is_empty() {
                continue;
            }
            if section == ".." {
                stack.pop();
            } else {
                stack.push(section.to_string());
            }
        }
        
        format!("/{}", stack.join("/"))
    }

    /// ## 739. Daily Temperatures
    ///
    /// https://leetcode.com/problems/daily-temperatures/
    ///
    /// Given an array of integers temperatures represents the daily temperatures, return an 
    /// array answer such that answer[i] is the number of days you have to wait after the ith 
    /// day to get a warmer temperature. If there is no future day for which this is possible, 
    /// keep answer[i] == 0 instead.
    ///
    /// Example 1:
    /// ----------
    /// Input: temperatures = [73,74,75,71,69,72,76,73]
    /// Output: [1,1,4,2,1,1,0,0]
    ///
    /// Example 2:
    /// ----------
    /// Input: temperatures = [30,40,50,60]
    /// Output: [1,1,1,0]
    ///
    /// Example 3:
    /// ----------
    /// Input: temperatures = [30,60,90]
    /// Output: [1,1,0]
    ///
    /// Constraints:
    /// ------------
    /// 1 <= temperatures.length <= 105
    /// 30 <= temperatures[i] <= 100
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![0; temperatures.len()];
        let mut stack: Vec<i32> = vec![];
        
        // Create a monotonic increasing queue
        // If current temp > stack top element, push current index to ans
        for i in 0..temperatures.len() {
            while !stack.is_empty() && temperatures[i] > temperatures[stack[stack.len() - 1] as usize] {
                let j = stack.pop().unwrap();
                ans[j as usize] = (i as i32 - j);
            }
            stack.push(i as i32);
        }

        ans
    }

    /// ## 1544. Make The String Great
    ///
    /// Given a string s of lower and upper case English letters.
    ///
    /// A good string is a string which doesn't have two adjacent characters s[i] and s[i + 1] where:
    /// - 0 <= i <= s.length - 2
    /// - s[i] is a lower-case letter and s[i + 1] is the same letter but in upper-case or vice-versa.
    ///
    /// To make the string good, you can choose two adjacent characters that make the string bad and 
    /// remove them. You can keep doing this until the string becomes good.
    ///
    /// Return the string after making it good. The answer is guaranteed to be unique under the given constraints.
    ///
    /// Notice that an empty string is also good.
    ///
    /// Example 1:
    /// ----------
    /// Input: s = "leEeetcode"
    /// Output: "leetcode"
    /// Explanation: In the first step, either you choose i = 1 or i = 2, both will result "leEeetcode" to be 
    /// reduced to "leetcode".
    ///
    /// Example 2:
    /// ----------
    /// Input: s = "abBAcC"
    /// Output: ""
    /// Explanation: We have many possible scenarios, and all lead to the same answer. For example:
    /// "abBAcC" --> "aAcC" --> "cC" --> ""
    /// "abBAcC" --> "abBA" --> "aA" --> ""
    ///
    /// Example 3:
    /// ----------
    /// Input: s = "s"
    /// Output: "s"
    ///
    ///
    /// Constraints:
    /// -----------
    /// 1 <= s.length <= 100
    /// s contains only lower and upper case English letters.
    /// 
    pub fn make_good(s: String) -> String {
        let mut stack = Vec::new();

        for ch in s.chars() {
            if stack.last().map_or(false, |&last| (last as i32 - ch as i32).abs() == 32) {
                stack.pop();
            } else {
                stack.push(ch);
            }
        }

        stack.iter().collect()
    }

    /// ## 239. Sliding Window Maximum
    ///
    /// https://leetcode.com/problems/sliding-window-maximum/
    ///
    /// You are given an array of integers nums, there is a sliding window of size k which is 
    /// moving from the very left of the array to the very right. You can only see the k numbers 
    /// in the window. Each time the sliding window moves right by one position.
    ///
    /// Return the max sliding window.
    ///
    /// Example 1:
    /// ----------
    /// Input: nums = [1,3,-1,-3,5,3,6,7], k = 3
    /// Output: [3,3,5,5,6,7]
    /// Explanation: 
    /// 
    /// Window position             |   Max
    /// ---------------             |  -----
    /// [1  3  -1] -3  5  3  6  7   |   3 
    /// 1  [3  -1  -3] 5  3  6  7    |   3
    /// 1  3  [-1  -3  5] 3  6  7    |   5
    /// 1  3  -1  [-3  5  3] 6  7    |   5
    /// 1  3  -1  -3  [5  3  6] 7    |   6
    /// 1  3  -1  -3  5  [3  6  7]   |   7
    ///
    /// Example 2:
    /// ----------
    /// Input: nums = [1], k = 1
    /// Output: [1]
    ///
    /// Constraints:
    /// ------------
    /// 1 <= nums.length <= 105
    /// -104 <= nums[i] <= 104
    /// 1 <= k <= nums.length
    /// 
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // Add first K element in the queue
        // Add to the answer only when the index >= k
        let mut ans: Vec<i32> = vec![];
        let mut deq: VecDeque<usize> = VecDeque::new();
        
        for i in 0..nums.len() {
            
            // Monotonic dequeue to keep track of the maximum values
            while !deq.is_empty() && nums[deq[deq.len() - 1]] < nums[i] {
                deq.pop_back();
            }

            deq.push_back(i);

            // If the front item of deque is out of window bound
            if (deq[0] + k as usize) == i {
                deq.pop_front();
            }

            // If the window size is reached
            if i >= k as usize - 1 {
                ans.push(nums[deq[0]]);
            }
        }
        ans
    }

    /// ## 1438. Longest Continuous Subarray With Absolute Diff Less Than or Equal to Limit
    ///
    /// https://leetcode.com/problems/longest-continuous-subarray-with-absolute-diff-less-than-or-equal-to-limit/
    ///
    /// Given an array of integers nums and an integer limit, return the size of the longest non-empty 
    /// subarray such that the absolute difference between any two elements of this subarray is less than 
    /// or equal to limit.
    ///
    /// Example 1:
    /// ----------
    /// Input: nums = [8,2,4,7], limit = 4
    /// Output: 2 
    /// Explanation: All subarrays are: 
    /// 
    /// - [8] with maximum absolute diff |8-8| = 0 <= 4.
    /// - [8,2] with maximum absolute diff |8-2| = 6 > 4. 
    /// - [8,2,4] with maximum absolute diff |8-2| = 6 > 4.
    /// - [8,2,4,7] with maximum absolute diff |8-2| = 6 > 4.
    /// - [2] with maximum absolute diff |2-2| = 0 <= 4.
    /// - [2,4] with maximum absolute diff |2-4| = 2 <= 4.
    /// - [2,4,7] with maximum absolute diff |2-7| = 5 > 4.
    /// - [4] with maximum absolute diff |4-4| = 0 <= 4.
    /// - [4,7] with maximum absolute diff |4-7| = 3 <= 4.
    /// - [7] with maximum absolute diff |7-7| = 0 <= 4. 
    /// Therefore, the size of the longest subarray is 2.
    /// 
    /// Example 2:
    /// ----------
    /// Input: nums = [10,1,2,4,7,2], limit = 5
    /// Output: 4 
    /// Explanation: The subarray [2,4,7,2] is the longest since the maximum absolute diff 
    /// is |2-7| = 5 <= 5.
    /// 
    /// Example 3:
    /// ----------
    /// Input: nums = [4,2,2,2,4,4,2,2], limit = 0
    /// Output: 3
    ///
    ///
    /// Constraints:
    /// ------------
    /// 1 <= nums.length <= 105
    /// 1 <= nums[i] <= 109
    /// 0 <= limit <= 109
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut incre: VecDeque<usize> = VecDeque::new();
        let mut decre: VecDeque<usize> = VecDeque::new();
        let mut ans: i32 = 0;  // size of the longest subarray
        let mut left: usize = 0;

        for right in 0..nums.len() {
            // If the last added item is more than current, pop the last added            
            while !incre.is_empty() && nums[incre[incre.len() - 1]] > nums[right] {
                incre.pop_back();
            }
            
            // If the last added item is less than current, pop the last added 
            while !decre.is_empty() && nums[decre[decre.len() - 1]] < nums[right] {
                decre.pop_back();
            }

            incre.push_back(right);
            decre.push_back(right);

            // Check for the constraint
            if nums[decre[0]] - nums[incre[0]] > limit {
                // remove the largest item
                if nums[left] == nums[decre[0]] {
                    decre.pop_front();
                }
                if nums[left] == nums[incre[0]] {
                    incre.pop_front();
                }
                left += 1;
            }

            ans = ans.max((right - left + 1) as i32);
        }

        ans
    }

    /// ## 496. Next Greater Element I
    ///  
    /// The next greater element of some element x in an array is the first greater element that is to 
    /// the right of x in the same array.
    ///
    /// You are given two distinct 0-indexed integer arrays nums1 and nums2, where nums1 is a subset of 
    /// nums2.
    ///
    /// For each 0 <= i < nums1.length, find the index j such that nums1[i] == nums2[j] and determine the 
    /// next greater element of nums2[j] in nums2. If there is no next greater element, then the answer 
    /// for this query is -1.
    ///
    /// Return an array ans of length nums1.length such that ans[i] is the next greater element as described 
    /// above.
    ///
    /// Example 1:
    /// ----------
    /// Input: nums1 = [4,1,2], nums2 = [1,3,4,2]
    /// Output: [-1,3,-1]
    /// Explanation: The next greater element for each value of nums1 is as follows:
    /// - 4 is underlined in nums2 = [1,3,4,2]. There is no next greater element, so the answer is -1.
    /// - 1 is underlined in nums2 = [1,3,4,2]. The next greater element is 3.
    /// - 2 is underlined in nums2 = [1,3,4,2]. There is no next greater element, so the answer is -1.
    /// 
    /// Example 2:
    ///
    /// Input: nums1 = [2,4], nums2 = [1,2,3,4]
    /// Output: [3,-1]
    /// Explanation: The next greater element for each value of nums1 is as follows:
    /// - 2 is underlined in nums2 = [1,2,3,4]. The next greater element is 3.
    /// - 4 is underlined in nums2 = [1,2,3,4]. There is no next greater element, so the answer is -1.
    ///
    ///
    /// Constraints:
    /// -----------
    /// 1 <= nums1.length <= nums2.length <= 1000
    /// 0 <= nums1[i], nums2[i] <= 104
    /// All integers in nums1 and nums2 are unique.
    /// All the integers of nums1 also appear in nums2.
    ///
    /// Follow up: Could you find an O(nums1.length + nums2.length) solution?
    ///
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<usize> = Vec::new();
        let mut greater: HashMap<i32, i32> = HashMap::new(); // save current -> next_higher
        for (i, num) in nums2.iter().enumerate() {
            while !stack.is_empty() && nums2[stack[stack.len() - 1]] < *num {
                let top: usize = stack.pop().unwrap();
                greater.insert(nums2[top as usize], *num);
            }
            stack.push(i);
        }

        nums1.iter().map(|&num| greater.get(&num).copied().unwrap_or(-1)).collect()
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplify_path() {
        let path = "/home/".to_string();
        let res = Solution::simplify_path(path);
        assert_eq!(res, "/home");

        let path = "/../".to_string();
        let res = Solution::simplify_path(path);
        assert_eq!(res, "/");

        let path = "/home//foo/".to_string();
        let res = Solution::simplify_path(path);
        assert_eq!(res, "/home/foo");
    }

    #[test]
    fn test_daily_temperatures() {
        let temperatures = vec![73,74,75,71,69,72,76,73];
        let res = Solution::daily_temperatures(temperatures);
        assert_eq!(res, vec![1,1,4,2,1,1,0,0]);

        let temperatures = vec![30,40,50,60];
        let res = Solution::daily_temperatures(temperatures);
        assert_eq!(res, vec![1,1,1,0]);
    }

    #[test]
    fn test_max_sliding_window() {
        let nums = vec![1,3,-1,-3,5,3,6,7];
        let k = 3;
        let res = Solution::max_sliding_window(nums, k);

        assert_eq!(res, vec![3,3,5,5,6,7]);

        let nums = vec![1];
        let k = 1;
        let res = Solution::max_sliding_window(nums, k);
        assert_eq!(res, vec![1]);

        let nums = vec![1,3,1,2,0,5];
        let k = 3;
        let res = Solution::max_sliding_window(nums, k);
        assert_eq!(res, vec![3,3,2, 5]);
    }

    #[test]
    fn test_next_greater_element() {
        let nums1 = vec![4,1,2];
        let nums2 = vec![1,3,4,2];
        let res = Solution::next_greater_element(nums1, nums2);
        assert_eq!(res, vec![-1,3,-1]); 

        let nums1 = vec![2,4]; 
        let nums2 = vec![1,2,3,4];
        let res = Solution::next_greater_element(nums1, nums2);
        assert_eq!(res, vec![3,-1]);
    }
}

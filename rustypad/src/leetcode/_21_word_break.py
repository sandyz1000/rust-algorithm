"""
https://leetcode.com/problems/word-break/

Given a string s and a dictionary of strings wordDict, return true if s can be 
segmented into a space-separated sequence of one or more dictionary words.

Note that the same word in the dictionary may be reused multiple times in the 
segmentation.

 
Example 1:

Input: s = "leetcode", wordDict = ["leet","code"]
Output: true
Explanation: Return true because "leetcode" can be segmented as "leet code".
Example 2:

Input: s = "applepenapple", wordDict = ["apple","pen"]
Output: true
Explanation: Return true because "applepenapple" can be segmented as "apple pen apple".
Note that you are allowed to reuse a dictionary word.
Example 3:

Input: s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
Output: false
 

Constraints:

1 <= s.length <= 300
1 <= wordDict.length <= 1000
1 <= wordDict[i].length <= 20
s and wordDict[i] consist of only lowercase English letters.
All the strings of wordDict are unique.

"""
from typing import List
from functools import cache


def wordBreak(s: str, wordDict: List[str]) -> bool:
    # Use level order traversal with index
    wordDict = set(wordDict)
    
    @cache
    def find_wordbreak(start_idx: int):
        # When every characeter is found
        if start_idx == len(s):
            return True
        
        # DFS rest of the character set when first portion is found
        for i in range(start_idx + 1, len(s)+1):
            if s[start_idx:i] in wordDict and find_wordbreak(i):
                return True
        return False
    
    return find_wordbreak(0)


if __name__ == "__main__":
    s = "leetcode"
    wordDict = ["leet", "code"]
    ans = wordBreak(s, wordDict)
    print(ans)
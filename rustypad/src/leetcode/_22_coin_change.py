"""
https://leetcode.com/problems/coin-change/

You are given an integer array coins representing coins of different denominations 
and an integer amount representing a total amount of money.

Return the fewest number of coins that you need to make up that amount. If that amount 
of money cannot be made up by any combination of the coins, return -1.

You may assume that you have an infinite number of each kind of coin.

Example 1:

Input: coins = [1,2,5], amount = 11
Output: 3
Explanation: 11 = 5 + 5 + 1
Example 2:

Input: coins = [2], amount = 3
Output: -1
Example 3:

Input: coins = [1], amount = 0
Output: 0

Constraints:
-----------
1 <= coins.length <= 12
1 <= coins[i] <= 231 - 1
0 <= amount <= 104

"""
from functools import cache
from typing import List
import sys


def coinChange(coins: List[int], amount: int) -> int:
    @cache
    def dp(target):
        if target == 0:
            return 0
        
        ans = sys.maxsize
        for coin in coins[::-1]:
            if coin > target:
                continue
            ans = min(ans, 1 + dp(target - coin))
        return ans
    
    ans = dp(amount)
    return ans if ans != sys.maxsize else -1


def main():
    coins = [1, 2, 5]
    amount = 11
    assert coinChange(coins, amount) == 3

    coins, amount = [2], 3
    assert coinChange(coins, amount) == -1

    coins, amount = [1], 0
    assert coinChange(coins, amount) == 0

if __name__ == "__main__":
    main()

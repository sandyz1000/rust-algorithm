"""
Number of ways to pair people
-----------------------------

Given that there are p people in a party. Each person can either join dance as a single 
individual or as a pair with any other. The task is to find the number of different ways 
in which p people can join the dance.

Examples:
Input : p = 3
Output : 4
Let the three people be P1, P2 and P3
Different ways are: {P1, P2, P3}, {{P1, P2}, P3},
{{P1, P3}, P2} and {{P2, P3}, P1}.

Input : p = 2
Output : 2
The groups are: {P1, P2} and {{P1, P2}}.

"""
# def find_ways_recurse(p: int) -> int:
#     if p == 1 or p == 2:
#         return 1 
#     for i in range(3, p+1):
#         pass
#     re

def findWays(p: int) -> int:
    dp = [0] * (p + 1)
    dp[1] = 1
    dp[2] = 2

    # Using the recurrence defined find count for different values of p.
    for i in range(3, p + 1):
        dp[i] = (dp[i - 1] + (i - 1) * dp[i - 2])
    return dp[p]


def main():
    p = 3
    print(findWays(p))


if __name__ == "__main__":
    main()

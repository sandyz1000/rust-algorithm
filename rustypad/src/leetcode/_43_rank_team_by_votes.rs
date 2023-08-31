#![allow(unused)]

struct Solution;


impl Solution {
    /// ## 1366. Rank Teams by Votes
    /// https://leetcode.com/problems/rank-teams-by-votes/description/
    ///
    /// In a special ranking system, each voter gives a rank from highest to lowest to all teams 
    /// participating in the competition.
    ///
    /// The ordering of teams is decided by who received the most position-one votes. If two or 
    /// more teams tie in the first position, we consider the second position to resolve the conflict, 
    /// if they tie again, we continue this process until the ties are resolved. If two or more teams 
    /// are still tied after considering all positions, we rank them alphabetically based on their team letter.
    ///
    /// You are given an array of strings votes which is the votes of all voters in the ranking systems. 
    /// Sort all teams according to the ranking system described above.
    ///
    /// Return a string of all teams sorted by the ranking system.
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let votes = vec!["ABC","ACB","ABC","ACB","ACB"].map(|s| s.to_string()).collect();
    /// assert_eq!(Solution::rank_teams(votes), "ACB");
    /// ```
    /// ### Explanation: ###
    /// Team A was ranked first place by 5 voters. No other team was voted as first place, so team A is the first team.
    /// Team B was ranked second by 2 voters and ranked third by 3 voters.
    /// Team C was ranked second by 3 voters and ranked third by 2 voters.
    /// As most of the voters ranked C second, team C is the second team, and team B is the third.
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let votes = vec!["WXYZ","XYZW"].map(|s| s.to_string()).collect();
    /// assert_eq!(Solution::rank_teams(votes), "XWYZ");
    /// ```
    /// ### Explanation: ###
    /// X is the winner due to the tie-breaking rule. X has the same votes as W for the first position, but X has one 
    /// vote in the second position, while W does not have any votes in the second position. 
    ///
    /// Example 3:
    /// ----------
    /// ```
    /// let votes = vec!["ZMNAGUEDSJYLBOPHRQICWFXTVK".to_string()];
    /// assert_eq!(Solution::rank_teams(votes), "ZMNAGUEDSJYLBOPHRQICWFXTVK".to_string());
    /// ```
    /// ### Explanation: ### 
    /// Only one voter, so their votes are used for the ranking.
    ///
    /// Constraints:
    /// ------------
    /// - 1 <= votes.length <= 1000
    /// - 1 <= votes[i].length <= 26
    /// - votes[i].length == votes[j].length for 0 <= i, j < votes.length.
    /// - votes[i][j] is an English uppercase letter.
    /// - All characters of votes[i] are unique.
    /// - All the characters that occur in votes[0] also occur in votes[j] where 1 <= j < votes.length.
    /// 
    pub fn rank_teams(votes: Vec<String>) -> String {
        // 2d grid of position and votes for each teams
        let mut count = vec![vec![0; 26]; votes[0].len()];
        for vote in votes.iter() {
            for (i, c) in vote.chars().enumerate() {
                count[i][c as usize - 'A' as usize] += 1;
            }
        }
        let mut teams: Vec<char> = votes[0].chars().collect();
        teams.sort_by(|a, b| {
            // Compare each position and votes for each team
            for item in &count {
                if item[*a as usize - 'A' as usize] != item[*b as usize - 'A' as usize] {
                    return item[*b as usize - 'A' as usize].cmp(&item[*a as usize - 'A' as usize]);
                }
            }
            a.cmp(b)
        });
        teams.iter().collect()
    }

    /// ## 1029. Two City Scheduling
    /// https://leetcode.com/problems/two-city-scheduling/
    ///
    /// A company is planning to interview 2n people. Given the array costs where costs[i] = [aCosti, bCosti], 
    /// the cost of flying the ith person to city a is aCosti, and the cost of flying the ith person to city b is bCosti.
    ///
    /// Return the minimum cost to fly every person to a city such that exactly n people arrive in each city.
    ///
    /// Example 1:
    /// -----------
    /// ```
    /// let costs = vec![vec![10,20],vec![30,200],vec![400,50],vec![30,20]];
    /// assert_eq!(Solution::two_city_sched_cost(costs), 110);
    /// ```
    /// **Explanation:** 
    /// The first person goes to city A for a cost of 10.
    /// The second person goes to city A for a cost of 30.
    /// The third person goes to city B for a cost of 50.
    /// The fourth person goes to city B for a cost of 20.
    ///
    /// The total minimum cost is 10 + 30 + 50 + 20 = 110 to have half the people interviewing in each city.
    /// 
    /// Example 2:
    /// -----------
    /// ```
    /// let costs = vec![vec![259,770],vec![448,54],vec![926,667],vec![184,139],vec![840,118],vec![577,469]];
    /// assert_eq!(Solution::two_city_sched_cost(costs), 1859);
    /// ```
    /// 
    /// Example 3:
    /// -----------
    /// ```
    /// let costs = vec![vec![515,563],vec![451,713],vec![537,709],vec![343,819],vec![855,779],vec![457,60],vec![650,359],vec![631,42]];
    /// assert_eq!(Solution::two_city_sched_cost(costs), 3086);
    /// ```
    ///
    /// Constraints:
    /// -----------
    /// - 2 * n == costs.length
    /// - 2 <= costs.length <= 100
    /// - costs.length is even.
    /// - 1 <= aCosti, bCosti <= 1000
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        // Sort by a gain which company has
        // by sending a person to city A and not to city B
        let mut costs = costs.clone();
        costs.sort_by_key(|x| x[0] - x[1]);

        let n = costs.len() / 2;
        let mut total = 0;

        // To optimize the company expenses,
        // send the first n persons to the city A
        // and the others to the city B
        for i in 0..n {
            total += costs[i][0] + costs[i + n][1];
        }

        total
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rank_teams() {
        let votes = vec!["ABC","ACB","ABC","ACB","ACB"].into_iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::rank_teams(votes), "ACB");

    }

    #[test]
    fn test_two() {
        let votes = vec!["WXYZ","XYZW"].into_iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::rank_teams(votes), "XWYZ");
    }

    #[test]
    fn test_one_votes() {
        let votes = vec!["ZMNAGUEDSJYLBOPHRQICWFXTVK".to_string()];
        assert_eq!(Solution::rank_teams(votes), "ZMNAGUEDSJYLBOPHRQICWFXTVK".to_string());
    }

    #[test]
    fn test_two_city_sched_cost() {
        let costs = vec![vec![10,20],vec![30,200],vec![400,50],vec![30,20]];
        assert_eq!(Solution::two_city_sched_cost(costs), 110);

        let costs = vec![vec![259,770],vec![448,54],vec![926,667],vec![184,139],vec![840,118],vec![577,469]];
        assert_eq!(Solution::two_city_sched_cost(costs), 1859);

        let costs = vec![
            vec![515,563],vec![451,713],vec![537,709],vec![343,819],vec![855,779],vec![457,60],vec![650,359],vec![631,42]
        ];
        assert_eq!(Solution::two_city_sched_cost(costs), 3086);
    }
}
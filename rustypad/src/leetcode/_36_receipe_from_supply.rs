#![allow(unused)]

use std::collections::{HashMap, HashSet, VecDeque};
struct Solution;

impl Solution {
    /// ## 2115. Find All Possible Recipes from Given Supplies
    /// https://leetcode.com/problems/find-all-possible-recipes-from-given-supplies/description/
    ///
    /// You have information about n different recipes. You are given a string array recipes and a 2D 
    /// string array ingredients. The ith recipe has the name recipes[i], and you can create it if you 
    /// have all the needed ingredients from ingredients[i]. Ingredients to a recipe may need to be 
    /// created from other recipes, i.e., ingredients[i] may contain a string that is in recipes.
    ///
    /// You are also given a string array supplies containing all the ingredients that you initially 
    /// have, and you have an infinite supply of all of them.
    ///
    /// Return a list of all the recipes that you can create. You may return the answer in any order.
    ///
    /// Note that two recipes may contain each other in their ingredients.
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let recipes = vec!["bread"]; let ingredients = vec![vec!["yeast","flour"]]; 
    /// let supplies = vec!["yeast","flour","corn"];
    /// assert_eq!(Solution::find_all_recipes(recipes, ingredients, supplies), vec!["bread"]);
    /// ```
    /// **Explanation:**
    /// We can create "bread" since we have the ingredients "yeast" and "flour".
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let recipes = vec!["bread","sandwich"]; let ingredients = vec![vec!["yeast","flour"],vec!["bread","meat"]];
    /// let supplies = vec!["yeast","flour","meat"];
    /// assert_eq!(Solution::find_all_recipes(recipes, ingredients, supplies), vec!["bread","sandwich"]);
    /// ```
    /// **Explanation:**
    /// We can create "bread" since we have the ingredients "yeast" and "flour".
    /// We can create "sandwich" since we have the ingredient "meat" and can create the ingredient "bread".
    ///
    /// Example 3:
    /// ----------
    /// ```
    /// let recipes = vec!["bread","sandwich","burger"];
    /// let ingredients = vec![vec!["yeast","flour"],vec!["bread","meat"],vec!["sandwich","meat","bread"]];
    /// let supplies = vec!["yeast","flour","meat"];
    /// assert_eq!(Solution::find_all_recipes(recipes, ingredients, supplies), vec!["bread","sandwich","burger"]);
    /// ```
    /// **Explanation:**
    /// We can create "bread" since we have the ingredients "yeast" and "flour".
    /// We can create "sandwich" since we have the ingredient "meat" and can create the ingredient "bread".
    /// We can create "burger" since we have the ingredient "meat" and can create the ingredients "bread" and "sandwich".
    ///
    /// Constraints:
    /// -----------
    /// - n == recipes.length == ingredients.length
    /// - 1 <= n <= 100
    /// - 1 <= ingredients[i].length, supplies.length <= 100
    /// - 1 <= recipes[i].length, ingredients[i][j].length, supplies[k].length <= 10
    /// - recipes[i], ingredients[i][j], and supplies[k] consist only of lowercase English letters.
    /// - All the values of recipes and supplies combined are unique.
    /// - Each ingredients[i] does not contain any duplicate values.
    ///
    pub fn find_all_recipes_topologically(
        recipes: Vec<String>, ingredients: Vec<Vec<String>>, supplies: Vec<String>
    ) -> Vec<String> {
        let mut res = Vec::new();
        let mut indegs = HashMap::new();
        let mut supplies: HashSet<_> = supplies.into_iter().collect();
        // Graph contains ingredients (not present in supplies) as keys and recipes as values
        let mut graph = HashMap::new();
        for (recipe, ings) in recipes.iter().zip(ingredients.into_iter()) {
            indegs.insert(recipe.as_str(), 0);
            for ing in ings {
                if !supplies.contains(&ing) {
                    graph.entry(ing).or_insert(vec![]).push(recipe.as_str());
                    *indegs.get_mut(recipe.as_str()).unwrap() += 1;
                }
            }
        }

        // Insert the recipe to queue where indegree is 0
        let mut dq = indegs.iter().fold(VecDeque::new(), |mut acc, (&s, cnt)| {
            if *cnt == 0 {
                acc.push_back(s);
            }
            acc
        });

        while !dq.is_empty() {
            let s = dq.pop_front().unwrap();
            res.push(s.to_owned());

            if let Some(v) = graph.get(s) {
                for rec in v.iter() {
                    let cnt = indegs.get_mut(rec).unwrap();
                    *cnt -= 1;
                    if *cnt == 0 {
                        dq.push_back(rec);
                    }
                }
            }
        }
        res
    }

    pub fn find_all_recipes(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        let mut graph: HashMap<_, _> = recipes.iter().zip(ingredients.into_iter()).collect();
        let mut visited: HashMap<_, _> = supplies.iter().map(|s| (s, true)).collect();

        recipes
            .iter()
            .filter(|s| Self::dfs(&graph, s, &mut visited))
            .cloned()
            .collect()
    }

    fn dfs<'a>(
        graph: &'a HashMap<&String, Vec<String>>,
        node: &'a String,
        visited: &mut HashMap<&'a String, bool>,
    ) -> bool {
        if !visited.contains_key(node) && !graph.contains_key(node) {
            return false;
        }
        if !visited.contains_key(node) {
            visited.insert(node, false);
            let res = graph
                .get(node)
                .unwrap()
                .iter()
                .all(|ing| Self::dfs(graph, ing, visited));
            *visited.get_mut(node).unwrap() = res;
        }
        *visited.get(node).unwrap()
    }
    
}


#[cfg(test)]
mod test {
    use super::*;
    fn convert_vec_vec_to_string(vec: Vec<Vec<&str>>) -> Vec<Vec<String>> {
        vec.into_iter().map(|inner| inner.into_iter().map(|s| s.to_string()).collect()).collect()
    }
    
    #[test]
    fn test_simple() {
        // Test case 3
        let recipes = vec!["bread".to_string()]; 
        let ingredients = convert_vec_vec_to_string(vec![vec!["yeast","flour"]]);
        let supplies = vec!["yeast","flour","corn"].into_iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::find_all_recipes(recipes, ingredients, supplies), vec!["bread"]);
    }

    #[test]
    fn test_find_all_recipes() {
        // Test case 1
        let recipes = vec!["bread","sandwich","burger"].into_iter().map(|s| s.to_string()).collect();
        let ingredients = convert_vec_vec_to_string(vec![
            vec!["yeast","flour"],
            vec!["bread","meat"],
            vec!["sandwich","meat","bread"]
        ]);
        let supplies = vec!["yeast","flour","meat"].into_iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::find_all_recipes(recipes, ingredients, supplies), vec!["bread","sandwich","burger"]);

        // Test case 2
        let recipes = vec!["bread","sandwich"].into_iter().map(|s| s.to_string()).collect();
        let ingredients = convert_vec_vec_to_string(vec![
            vec!["yeast","flour"],
            vec!["bread","meat"]
        ]);
        let supplies = vec!["yeast","flour","meat"].into_iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::find_all_recipes(recipes, ingredients, supplies), vec!["bread","sandwich"]);

    }
}
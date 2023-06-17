#![allow(unused)]

struct Solution;

impl Solution {
    /// ## 2126. Destroying Asteroids
    ///
    /// You are given an integer mass, which represents the original mass of a planet. You are further given an 
    /// integer array asteroids, where asteroids[i] is the mass of the ith asteroid.
    ///
    /// You can arrange for the planet to collide with the asteroids in any arbitrary order. If the mass of the 
    /// planet is greater than or equal to the mass of the asteroid, the asteroid is destroyed and the planet gains 
    /// the mass of the asteroid. Otherwise, the planet is destroyed.
    ///
    /// Return true if all asteroids can be destroyed. Otherwise, return false.
    ///
    /// Example 1:
    /// ----------
    /// - Input: mass = 10, asteroids = [3,9,19,5,21]
    /// - Output: true
    ///
    /// Explanation: One way to order the asteroids is [9,19,5,3,21]:
    /// - The planet collides with the asteroid with a mass of 9. New planet mass: 10 + 9 = 19
    /// - The planet collides with the asteroid with a mass of 19. New planet mass: 19 + 19 = 38
    /// - The planet collides with the asteroid with a mass of 5. New planet mass: 38 + 5 = 43
    /// - The planet collides with the asteroid with a mass of 3. New planet mass: 43 + 3 = 46
    /// - The planet collides with the asteroid with a mass of 21. New planet mass: 46 + 21 = 67
    /// All asteroids are destroyed.
    ///
    /// Example 2:
    /// ----------
    /// - Input: mass = 5, asteroids = [4,9,23,4]
    /// - Output: false
    /// 
    /// Explanation: 
    /// The planet cannot ever gain enough mass to destroy the asteroid with a mass of 23.
    /// After the planet destroys the other asteroids, it will have a mass of 5 + 4 + 9 + 4 = 22.
    /// This is less than 23, so a collision would not destroy the last asteroid.
    ///
    /// Constraints:
    /// -----------
    /// 1 <= mass <= 105
    /// 1 <= asteroids.length <= 105
    /// 1 <= asteroids[i] <= 105
    ///
    pub fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool {
        unimplemented!()   
    }

    /// ## 881. Boats to Save People
    /// https://leetcode.com/problems/boats-to-save-people/
    /// 
    /// You are given an array people where people[i] is the weight of the ith person, and an infinite number of 
    /// boats where each boat can carry a maximum weight of limit. Each boat carries at most two people at the same 
    /// time, provided the sum of the weight of those people is at most limit.
    ///
    /// Return the minimum number of boats to carry every given person.
    ///
    /// Example 1:
    /// ----------
    /// - Input: people = [1,2], limit = 3
    /// - Output: 1
    /// Explanation: 1 boat (1, 2)
    ///
    /// Example 2:
    /// ----------
    /// - Input: people = [3,2,2,1], limit = 3
    /// - Output: 3
    /// Explanation: 3 boats (1, 2), (2) and (3)
    ///
    /// Example 3:
    /// ----------
    /// Input: people = [3,5,3,4], limit = 5
    /// Output: 4
    /// Explanation: 4 boats (3), (3), (4), (5)
    ///
    ///
    /// Constraints:
    /// -----------
    /// 1 <= people.length <= 5 * 104
    /// 1 <= people[i] <= limit <= 3 * 104
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        unimplemented!()   
    }

    /// ## 1323. Maximum 69 Number
    ///
    /// You are given a positive integer num consisting only of digits 6 and 9.
    ///
    /// Return the maximum number you can get by changing at most one digit (6 becomes 9, and 9 becomes 6).
    ///
    /// Example 1:
    /// ----------
    /// - Input: num = 9669
    /// - Output: 9969
    ///
    /// Explanation: 
    /// Changing the first digit results in 6669.
    /// Changing the second digit results in 9969.
    /// Changing the third digit results in 9699.
    /// Changing the fourth digit results in 9666.
    /// The maximum number is 9969.
    ///
    /// Example 2:
    /// ----------
    /// - Input: num = 9996
    /// - Output: 9999
    /// Explanation: Changing the last digit 6 to 9 results in the maximum number.
    ///
    /// Example 3:
    /// ----------
    /// - Input: num = 9999
    /// - Output: 9999
    /// 
    /// Explanation: It is better not to apply any change.
    ///
    /// Constraints:
    /// -----------
    /// 1 <= num <= 104
    /// num consists of only 6 and 9 digits.
    pub fn maximum69_number(num: i32) -> i32 {
        unimplemented!()    
    }

    /// ## 1710. Maximum Units on a Truck
    ///
    /// You are assigned to put some amount of boxes onto one truck. You are given a 2D array boxTypes, 
    /// where boxTypes[i] = [numberOfBoxesi, numberOfUnitsPerBoxi]:
    ///
    /// - numberOfBoxesi is the number of boxes of type i.
    /// - numberOfUnitsPerBoxi is the number of units in each box of the type i.
    ///
    /// You are also given an integer truckSize, which is the maximum number of boxes that can be put on 
    /// the truck. You can choose any boxes to put on the truck as long as the number of boxes does not exceed truckSize.
    ///
    /// Return the maximum total number of units that can be put on the truck.
    ///
    /// Example 1:
    /// ----------
    /// - Input: boxTypes = [[1,3],[2,2],[3,1]], truckSize = 4
    /// - Output: 8
    ///
    /// Explanation: There are:
    /// - 1 box of the first type that contains 3 units.
    /// - 2 boxes of the second type that contain 2 units each.
    /// - 3 boxes of the third type that contain 1 unit each.
    /// You can take all the boxes of the first and second types, and one box of the third type.
    /// The total number of units will be = (1 * 3) + (2 * 2) + (1 * 1) = 8.
    ///
    /// Example 2:
    /// ----------
    /// - Input: boxTypes = [[5,10],[2,5],[4,7],[3,9]], truckSize = 10
    /// - Output: 91
    ///
    /// Constraints:
    /// -----------
    /// 1 <= boxTypes.length <= 1000
    /// 1 <= numberOfBoxesi, numberOfUnitsPerBoxi <= 1000
    /// 1 <= truckSize <= 106
    ///
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        unimplemented!()
    }

    /// ## 1196. How Many Apples Can You Put into the Basket
    ///
    ///
    /// You have some apples and a basket that can carry up to 5000 units of weight.
    ///
    /// Given an integer array weight where weight[i] is the weight of the ith apple, return the maximum 
    /// number of apples you can put in the basket.
    ///
    /// Example 1:
    /// ----------
    /// - Input: weight = [100,200,150,1000]
    /// - Output: 4
    ///
    /// Explanation: All 4 apples can be carried by the basket since their sum of weights is 1450.
    ///
    /// Example 2:
    /// ----------
    /// - Input: weight = [900,950,800,1000,700,800]
    /// - Output: 5
    ///
    /// Explanation: The sum of weights of the 6 apples exceeds 5000 so we choose any 5 of them.
    ///
    /// Constraints:
    /// -----------
    /// 1 <= weight.length <= 103
    /// 1 <= weight[i] <= 103
    pub fn max_number_of_apples(weight: Vec<i32>) -> i32 {
        unimplemented!()   
    }

    /// ## Reduce Array Size to The Half
    ///
    /// You are given an integer array arr. You can choose a set of integers and remove all the occurrences 
    /// of these integers in the array.
    ///
    /// Return the minimum size of the set so that at least half of the integers of the array are removed.
    ///
    ///
    /// Example 1:
    /// ----------
    /// - Input: arr = [3,3,3,3,5,5,5,2,2,7]
    /// - Output: 2
    /// - Explanation: 
    /// Choosing {3,7} will make the new array [5,5,5,2,2] which has size 5 (i.e equal to half of 
    /// the size of the old array). 
    /// Possible sets of size 2 are {3,5},{3,2},{5,2}.
    /// Choosing set {2,7} is not possible as it will make the new array [3,3,3,3,5,5,5] which has a size greater 
    /// than half of the size of the old array.
    ///
    /// Example 2:
    /// ----------
    /// - Input: arr = [7,7,7,7,7,7]
    /// - Output: 1
    /// - Explanation: The only possible set you can choose is {7}. This will make the new array empty.
    ///
    /// Constraints:
    /// ----------------
    /// 2 <= arr.length <= 105
    /// arr.length is even.
    /// 1 <= arr[i] <= 105
    ///
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        unimplemented!()   
    }

}


mod test {
    use super::*;
    
    #[test]
    fn test_asteroids_destroyed() {
        
    }

    #[test]
    fn test_num_rescue_boats() {

    }
}
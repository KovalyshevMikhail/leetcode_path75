/// Kids With the Greatest Number of Candies
///
/// There are n kids with candies. You are given an integer array candies,
/// where each candies[i] represents the number of candies the ith kid has,
/// and an integer extraCandies, denoting the number of extra candies that you have.
///
/// Return a boolean array result of length n, where result[i] is true if,
/// after giving the ith kid all the extraCandies, they will have the greatest number of candies among all the kids,
/// or false otherwise.
///
/// Note that multiple kids can have the greatest number of candies.
///
/// Example 1:
///
/// Input: candies = [2,3,5,1,3], extraCandies = 3
/// Output: [true,true,true,false,true]
/// Explanation: If you give all extraCandies to:
/// - Kid 1, they will have 2 + 3 = 5 candies, which is the greatest among the kids.
/// - Kid 2, they will have 3 + 3 = 6 candies, which is the greatest among the kids.
/// - Kid 3, they will have 5 + 3 = 8 candies, which is the greatest among the kids.
/// - Kid 4, they will have 1 + 3 = 4 candies, which is not the greatest among the kids.
/// - Kid 5, they will have 3 + 3 = 6 candies, which is the greatest among the kids.
///

pub fn _kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    if !candies.is_empty() {
        let max_candies = candies.iter().max().unwrap();
        let result = candies
            .iter()
            .map(|amount| (amount + extra_candies) >= *max_candies)
            .collect::<Vec<bool>>();

        result
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        println!("\n===================== ROUND 1 ===========================\n");
        let candies = vec![2, 3, 5, 1, 3];
        let extra_candies = 3;
        let check = vec![true, true, true, false, true];

        let result = _kids_with_candies(candies, extra_candies);

        println!("{:?} == {:?}", result, check);

        assert!(result.eq(&check));
    }
}

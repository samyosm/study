/* // O(n^2) solution
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (ia, &a) in nums.iter().enumerate() {
        for (ib, &b) in nums.iter().enumerate() {
            if a + b == target && ia != ib {
                return vec![a, b];
            }
        }
    }

    panic!("No result found!");
}
*/

use std::collections::HashMap;

// O(n) solution
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();

    for i in 0..nums.len() {
        if let Some(&j) = map.get(&(target - nums.get(i).unwrap())) {
            return vec![j as i32, i as i32];
        }

        map.insert(nums[i], i);
    }

    panic!("No result found!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let result = two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn case_2() {
        let result = two_sum(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn case_3() {
        let result = two_sum(vec![3, 3], 6);
        assert_eq!(result, vec![0, 1]);
    }
}

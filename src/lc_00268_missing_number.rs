#![allow(dead_code)]
use std::collections::HashSet;
// sum method
pub fn missing_number(nums: Vec<i32>) -> i32 {
    (1..=nums.len()).sum::<usize>() as i32 - nums.into_iter().sum::<i32>()
}

// brute force
pub fn missing_number2(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    let v2: Vec<i32> = (0..=n).collect();
    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();

    for i in nums.iter() {
        set1.insert(i);
    }

    for i in v2.iter() {
        set2.insert(i);
    }

    let mut rs = set2.difference(&set1);

    return **rs.next().unwrap();
}

// Method using xor ^
pub fn missing_number3(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc ^ (i + 1) as i32 ^ x)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn ex1() {
        //Input: nums = [3,0,1]
        // Output: 2
        let nums = vec![3, 0, 1];
        let output = 2;
        assert_eq!(missing_number(nums), output);
    }

    #[test]
    fn ex2() {
        //Input: nums = [0,1]
        // Output: 2
        let nums = vec![0, 1];
        let output = 2;
        assert_eq!(missing_number(nums), output);
    }

    #[test]
    fn ex3() {
        //Input: nums =[9,6,4,2,3,5,7,0,1]
        // Output: 8
        let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        let output = 8;
        assert_eq!(missing_number(nums), output);
    }
}

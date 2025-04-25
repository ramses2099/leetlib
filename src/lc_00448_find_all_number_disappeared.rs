#![allow(dead_code)]
use std::collections::HashSet;

// brute force
pub fn find_disappeared_numbers1(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len() as i32;
    let v2: Vec<i32> = (1..=n).collect();

    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();

    for i in nums.iter() {
        set1.insert(i);
    }

    for i in v2.iter() {
        set2.insert(i);
    }

    let rs = set2.difference(&set1);
    let mut rs2: Vec<i32> = Vec::new();

    for i in rs {
        rs2.push(**i);
    }

    return rs2;
}

pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len() as i32;
    let mut set = HashSet::new();

    for i in nums {
        set.insert(i);
    }

    (1..=n).filter(|i| !set.contains(i)).collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn ex1() {
        // Input: nums = [4,3,2,7,8,2,3,1]
        // Output: [5,6]
        let nums = Vec::from([4,3,2,7,8,2,3,1]);
        let output = Vec::from([5, 6]);
        assert_eq!(find_disappeared_numbers(nums), output);
    }

    #[test]
    fn ex2() {
        // Input: nums = [1,1]
        // Output: [2]
        let nums = Vec::from([1,1]);
        let output = Vec::from([2]);
        assert_eq!(find_disappeared_numbers(nums), output);
    }
}

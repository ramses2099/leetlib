#![allow(dead_code)]
use std::collections::HashMap;

pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut rs: Vec<i32> = Vec::new();
    let mut map = HashMap::new();
    let mut temp = nums.clone();
    temp.sort();

    for (i, v) in temp.iter().enumerate() {
        if map.contains_key(v) {
            continue;
        }
        map.insert(v, i);
    }

    for i in nums.iter() {
        rs.push(map[i] as i32);
    }
    rs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let nums = vec![8, 1, 2, 2, 3];
        let outpt = vec![4, 0, 1, 1, 3];

        assert_eq!(smaller_numbers_than_current(nums), outpt);
    }

    #[test]
    fn ex2() {
        let nums = vec![7, 7, 7, 7];
        let outpt = vec![0, 0, 0, 0];

        assert_eq!(smaller_numbers_than_current(nums), outpt);
    }
}
 
#![allow(dead_code)]
use std::collections::HashSet;

// Method using xor ^

fn contains_duplicate(nums: &mut Vec<i32>) -> bool {
    let mut set = HashSet::new();

    for i in nums {
        if !set.insert(i) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn ex1() {
        //Input: nums = [3,0,1]
        // Output: 2
        let mut nums = vec![1, 2, 3, 1];
        //ex 1
        assert_eq!(contains_duplicate(&mut nums), true);
    }

    #[test]
    fn ex2() {
        //Input: nums = [0,1]
        // Output: 2
        let mut nums = vec![1, 2, 3, 4];
        //ex 2
        assert_eq!(contains_duplicate(&mut nums), false);
    }

    #[test]
    fn ex3() {
        //Input: nums =[9,6,4,2,3,5,7,0,1]
        // Output: 8
        let mut nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        //ex 3
        assert_eq!(contains_duplicate(&mut nums), true);
    }
}

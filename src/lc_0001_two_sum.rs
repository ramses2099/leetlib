#![allow(dead_code)]
use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_map = HashMap::new();
    for (i, &num) in nums.iter().enumerate(){
        let complement = target - num;
        if let Some(&index) = num_map.get(&complement){
            return vec![index as i32, i as i32];
        }
        num_map.insert(num,i );
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = vec![2, 7, 11, 15];
        let target = 9;
        let output = vec![0, 1];

        assert_eq!(two_sum(input, target), output);
    }

    #[test]
    fn ex2() {
        let input = vec![3, 2, 4];
        let target = 6;
        let output = vec![1, 2];

        assert_eq!(two_sum(input, target), output);
    }

    #[test]
    fn ex3() {
        let input = vec![3, 3];
        let target = 6;
        let output = vec![0, 1];

        assert_eq!(two_sum(input, target), output);
    }
}

#![allow(dead_code)]
use std::collections::HashMap;


pub fn counter(s: String) -> HashMap<char, i32> {
    let mut map = HashMap::new();
    for c in s.chars() {
        if map.contains_key(&c) {
            *map.get_mut(&c).unwrap() += 1;
        } else {
            map.insert(c, 1);
        }
    }
    map
}

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut map = counter(s);

    for l in t.chars() {
        if !map.contains_key(&l) || *map.get(&l).unwrap() == 0 {
            return false;
        }
        *map.get_mut(&l).unwrap() -= 1;
    }

    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();
    
        assert_eq!(is_anagram(s, t), true);
    }

    #[test]
    fn ex2() {
        let s = "rat".to_string();
        let t = "car".to_string();
    
        assert_eq!(is_anagram(s, t), false);
    }

   
}

#![allow(dead_code)]
pub fn is_valid(s: String) -> bool {
    let mut stack = "".to_string();

    for ch in s.chars() {
        match ch {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            ')' | ']' | '}' => {
                if let Some(m) = stack.pop() {
                    if m != ch {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => {
                return false;
            }
        }
    }

    0 == stack.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = "()".to_string();
        let output = true;

        assert_eq!(is_valid(input), output);
    }

    #[test]
    fn ex2() {
        let input = "()[]{}".to_string();
        let output = true;

        assert_eq!(is_valid(input), output);
    }

    #[test]
    fn ex3() {
        let input = "(]".to_string();
        let output = false;

        assert_eq!(is_valid(input), output);
    }

    #[test]
    fn ex4() {
        let input = "([])".to_string();
        let output = true;

        assert_eq!(is_valid(input), output);
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut prev, mut curr) = (None, head);
        while let Some(mut node) = curr {
            curr = node.next;

            node.next = prev;

            prev = Some(node);
        }
        prev
    }
    //
    pub fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut list = ListNode::new(0);
        for i in (0..vec.len()).rev() {
            let mut new_node = ListNode::new(vec[i]);
            new_node.next = list.next.take();
            list.next = Some(Box::new(new_node));
        }
        list.next
    }
    //
    pub fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut r = Vec::new();
        let mut current = head;
        while let Some(node) = current {
            r.push(node.val);
            current = node.next;
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = vec![1, 2, 3, 4, 5];
        let output = vec![5, 4, 3, 2, 1];
        let head = Solution::from_vec(input);
        let rev = Solution::reverse_list(head);
        let rev_ouput = Solution::to_vec(rev);
        assert_eq!(rev_ouput, output);
    }

    #[test]
    fn ex2() {
        let input = vec![1, 2];
        let output = vec![2, 1];
        let head = Solution::from_vec(input);
        let rev = Solution::reverse_list(head);
        let rev_ouput = Solution::to_vec(rev);
        assert_eq!(rev_ouput, output);
    }
}

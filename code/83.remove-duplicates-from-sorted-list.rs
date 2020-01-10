// Definition for singly-linked list.
// struct Solution;
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }
// mod ll;
// use ll::*;
impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut cur = head.as_mut()?;
        while let Some(next) = cur.next.as_mut() {
            if cur.val != next.val {
                cur = cur.next.as_mut()?;
                continue;
            }
            let b2n = std::mem::replace(&mut next.next, None);
            std::mem::replace(&mut cur.next, b2n);
        }

        head
    }
}

// fn main() {
//     let r = Solution::delete_duplicates(ListNode::from(vec![1, 2, 2, 3]));
//     println!("{:?}", r);
// }

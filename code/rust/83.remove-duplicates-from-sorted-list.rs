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
mod ll;
use ll::*;
impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ptr = head.as_mut()?;

        while let Some(next) = ptr.next.as_mut() {
            if ptr.val == next.val {
                // let tmp = std::mem::replace(&mut next.next, None);
                // std::mem::replace(&mut ptr.next, tmp);
                ptr.next = next.next.take();
            } else {
                ptr = ptr.next.as_mut()?;
            }
        }

        head
    }
}

fn main() {
    let r = Solution::delete_duplicates(ListNode::from(vec![1, 2, 2, 3]));
    println!("{:#?}", r);
}

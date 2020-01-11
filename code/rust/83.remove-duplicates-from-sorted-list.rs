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
        let mut ptr = &mut head;
        let mut _nxt: Option<Box<ListNode>> = None;

        while ptr.is_some() && ptr.as_ref()?.next.is_some() {
            if ptr.as_ref()?.val == ptr.as_ref()?.next.as_ref()?.val {
                // make sure not directly use next.next
                _nxt = ptr.as_mut()?.next.take();
                ptr.as_mut()?.next = _nxt.as_mut()?.next.take();
            }
            ptr = &mut ptr.as_mut()?.next;
        }

        head
    }
}

fn main() {
    let r = Solution::delete_duplicates(ListNode::from(vec![1, 2, 2, 3]));
    println!("{:#?}", r);
}

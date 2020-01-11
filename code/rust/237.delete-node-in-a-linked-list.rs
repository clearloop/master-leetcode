// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
mod ll;
use ll::*;
impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut ptr = &mut head;

        while ptr.is_some() {
            // replace current value in memory
            if ptr.as_ref()?.val == val {
                *ptr = ptr.as_mut()?.next.take();
            } else {
                ptr = &mut ptr.as_mut()?.next;
            }
        }

        head
    }
}

fn main() {
    let node = ListNode::from(vec![1, 2, 6, 3, 4, 5, 6]);
    let res = Solution::remove_elements(node, 6);
    println!("{:?}", res);
}

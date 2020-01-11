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
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ret = None;
        let mut _nxt = None;

        while head.is_some() {
            _nxt = head.as_mut()?.next.take();
            head.as_mut()?.next = ret;
            ret = head;

            // Reverse
            //
            // Old ListNode: "***>>"
            // Ret ListNode: ">>>"
            head = _nxt;
        }

        ret
    }
}

fn main() {
    let n = ListNode::from(vec![1, 2, 3, 4, 5]);
    let r = Solution::reverse_list(n);
    println!("{:?}", r);
}

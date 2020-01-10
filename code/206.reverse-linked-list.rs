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
        let mut ptr = &mut head;
        let mut ret: Option<Box<ListNode>> = None;

        while ptr.is_some() {
            let tmp = ptr.as_mut()?;
            tmp.next = ret;
            ret = tmp.take();

            ptr = &mut ptr.as_mut()?.next;
        }

        ret
    }
}

fn main() {
    let n = ListNode::from(vec![1, 2, 3, 4, 5]);
    let r = Solution::reverse_list(n);
    println!("{:?}", r);
}

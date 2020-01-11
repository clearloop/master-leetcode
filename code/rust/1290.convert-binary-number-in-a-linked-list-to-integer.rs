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
// mod ll;
// use ll::*;
impl Solution {
    pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
        let mut ret = 0;
        while head.is_some() {
            ret = (ret << 1) | head.as_ref().unwrap().val;
            head = head.unwrap().next;
        }

        ret
    }
}

// fn main() {
//     let ln = ListNode::from(vec![1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0]);
//     let r = Solution::get_decimal_value(ln);
//     println!("{:?}", r);
// }

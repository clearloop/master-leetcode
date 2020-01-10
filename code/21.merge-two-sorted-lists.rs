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
impl Solution {
    /// O(n)
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        use std::mem;
        let mut l1 = l1;
        let mut l2 = l2;

        let mut ls = &mut l1;
        let ll = &mut l2;

        while ll.is_some() {
            if ls.is_none() || ls.as_ref()?.val > ll.as_ref()?.val {
                mem::swap(ls, ll);
            }

            if ls.is_some() {
                ls = &mut ls.as_mut()?.next
            }
        }

        l1
    }
}
// test
// fn main() {
//     let r = Solution::merge_two_lists(
//         Some(Box::new(ListNode::new(0))),
//         Some(Box::new(ListNode::new(1))),
//     );
//     println!("{:?}", r);
// }

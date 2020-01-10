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
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        use std::mem;

        let mut pa = &mut l1;
        let pb = &mut l2;

        while pb.is_some() {
            if pa.is_none() || pa.as_ref()?.val > pb.as_ref()?.val {
                mem::swap(pa, pb);
            } else {
                pa = &mut pa.as_mut()?.next;
            }
        }

        l1
    }
}

// fn main() {
//     let l1 = ListNode::from(vec![1, 2, 4]);
//     let l2 = ListNode::from(vec![1, 3, 4]);
//     let r = Solution::merge_two_lists(l1, l2);
//     println!("{:#?}", r);
// }

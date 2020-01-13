// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::i32::{MAX, MIN};
use std::rc::Rc;
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::h(&root, MIN, MAX).1
    }

    fn h(r: &Option<Rc<RefCell<TreeNode>>>, last: i32, res: i32) -> (i32, i32) {
        if let Some(r) = r {
            let r = r.borrow();
            let (mut last, mut res) = Self::h(&r.left.clone(), last, res);
            res = res.min(r.val.saturating_sub(last));
            last = r.val;
            return Self::h(&r.right.clone(), last, res);
        }
        return (last, res);
    }
}

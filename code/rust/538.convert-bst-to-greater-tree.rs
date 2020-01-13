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
use std::rc::Rc;
impl Solution {
    pub fn convert_bst(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum = 0;
        Self::h(&mut root, &mut sum);
        root
    }

    fn h(r: &mut Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if let Some(r) = r {
            let mut r = r.borrow_mut();
            Self::h(&mut r.right, sum);
            r.val += *sum;
            *sum = r.val;
            Self::h(&mut r.left, sum);
        }
    }
}

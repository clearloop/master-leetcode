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
use std::i32;
use std::rc::Rc;
impl Solution {
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min = i32::MAX;
        let mut pre = i32::MIN;
        Self::h(&root, &mut pre, &mut min);
        min
    }

    fn h(r: &Option<Rc<RefCell<TreeNode>>>, pre: &mut i32, min: &mut i32) {
        if let Some(r) = r {
            let r = r.borrow();
            Self::h(&r.left, pre, min);
            *min = (r.val.saturating_sub(*pre)).min(*min);
            *pre = r.val; // !important;
            Self::h(&r.right, pre, min);
        }
    }
}

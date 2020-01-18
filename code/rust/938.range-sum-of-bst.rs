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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
        let mut sum = 0;
        Self::h(&root, &mut sum, l, r);
        sum
    }

    fn h(t: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32, l: i32, r: i32) {
        if let Some(t) = t {
            let t = t.borrow();
            if t.val <= r && t.val >= l {
                *sum += t.val;
            }

            Self::h(&t.left, sum, l, r);
            Self::h(&t.right, sum, l, r);
        }
    }
}

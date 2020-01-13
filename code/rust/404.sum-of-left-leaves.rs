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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        Self::h(&root, &mut sum);
        sum
    }

    fn h(r: &Option<Rc<RefCell<TreeNode>>>, s: &mut i32) {
        if let Some(r) = r {
            let r = r.borrow();

            // add
            if let Some(l) = &r.left {
                let l = l.borrow();
                if l.left.is_none() && l.right.is_none() {
                    *s += l.val;
                }
            }

            // rescursion
            Self::h(&r.left, s);
            Self::h(&r.right, s);
        }
    }
}

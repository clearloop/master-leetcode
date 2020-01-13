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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut res = 0;
        Self::h(&mut res, sum, &root);
        if let Some(r) = root {
            let r = r.borrow();
            res += Self::path_sum(r.left.clone(), sum);
            res += Self::path_sum(r.right.clone(), sum);
        }
        res
    }

    fn h(r: &mut i32, s: i32, t: &Option<Rc<RefCell<TreeNode>>>) {
        if let Some(t) = t {
            let t = t.borrow();
            if t.val == s {
                *r += 1;
            }

            Self::h(r, s - t.val, &t.left);
            Self::h(r, s - t.val, &t.right);
        }
    }
}

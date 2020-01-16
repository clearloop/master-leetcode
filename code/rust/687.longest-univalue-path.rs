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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut c = 0;
        Self::h(&root, &mut c);
        c
    }

    fn h(r: &Option<Rc<RefCell<TreeNode>>>, c: &mut i32) -> i32 {
        if let Some(r) = r {
            let r = r.borrow();
            let mut tlc = Self::h(&r.left, c);
            let mut trc = Self::h(&r.right, c);

            let (mut lc, mut rc) = (0, 0);
            if let Some(tl) = &r.left {
                let tl = tl.borrow();
                if tl.val == r.val {
                    lc = tlc + 1;
                }
            }

            if let Some(tr) = &r.right {
                let tr = tr.borrow();
                if tr.val == r.val {
                    rc = trc + 1;
                }
            }

            *c = *c.max(&mut (lc + rc));
            lc.max(rc)
        } else {
            0
        }
    }
}

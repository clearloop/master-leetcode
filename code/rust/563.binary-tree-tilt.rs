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
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut s = 0;
        Self::h(&root, &mut s);
        s
    }

    fn h(n: &Option<Rc<RefCell<TreeNode>>>, s: &mut i32) -> i32 {
        if let Some(n) = n {
            let n = n.borrow();
            let l = Self::h(&n.left, s);
            let r = Self::h(&n.right, s);
            *s += (l - r).abs();
            n.val + l + r
        } else {
            0
        }
    }
}

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
    pub fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::h(&mut root);
        root
    }

    fn h(r: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(r) = r {
            let mut r = r.borrow_mut();
            Self::h(&mut r.left);
            Self::h(&mut r.right);

            let lt = std::mem::replace(&mut r.left, None);
            let rt = std::mem::replace(&mut r.right, lt);
            std::mem::replace(&mut r.left, rt);
        }
    }
}

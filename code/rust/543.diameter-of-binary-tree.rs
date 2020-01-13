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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut d = 1;
        Self::h(&root, &mut d);
        d - 1
    }

    fn h(r: &Option<Rc<RefCell<TreeNode>>>, d: &mut i32) -> i32 {
        if let Some(r) = r {
            let r = r.borrow();
            let p = Self::h(&r.left, d);
            let q = Self::h(&r.right, d);
            *d = d.to_owned().max(p + q + 1);
            1 + p.max(q)
        } else {
            0
        }
    }
}

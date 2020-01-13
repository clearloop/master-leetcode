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
    pub fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::h(&s, &t)
    }

    fn h(s: &Option<Rc<RefCell<TreeNode>>>, t: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if s == t {
            return true;
        } else if s.is_none() {
            return false;
        }

        let s = s.as_ref().unwrap().borrow();
        Self::h(&s.left, t) || Self::h(&s.right, t)
    }
}

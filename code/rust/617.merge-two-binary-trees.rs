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
    pub fn merge_trees(
        mut t1: Option<Rc<RefCell<TreeNode>>>,
        mut t2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::h(&mut t1, &mut t2);
        t1
    }

    fn h(p: &mut Option<Rc<RefCell<TreeNode>>>, q: &mut Option<Rc<RefCell<TreeNode>>>) {
        if p.is_some() && q.is_some() {
            let (mut p, mut q) = (
                p.as_mut().unwrap().borrow_mut(),
                q.as_mut().unwrap().borrow_mut(),
            );
            p.val += q.val;
            Self::h(&mut p.left, &mut q.left);
            Self::h(&mut p.right, &mut q.right);
        } else if p.is_none() && q.is_some() {
            *p = q.take();
        }
    }
}

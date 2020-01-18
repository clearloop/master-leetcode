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
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut res = true;
        Self::h(&root, &mut res);
        res
    }

    fn h(t: &Option<Rc<RefCell<TreeNode>>>, res: &mut bool) {
        if let Some(t) = t {
            let t = t.borrow();
            if let Some(l) = &t.left {
                if l.borrow().val != t.val {
                    *res = false;
                }
                Self::h(&t.left, res);
            }

            if let Some(r) = &t.right {
                if r.borrow().val != t.val {
                    *res = false;
                }
                Self::h(&t.right, res);
            }
        }
    }
}

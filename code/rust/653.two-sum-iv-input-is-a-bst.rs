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
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut m = HashMap::<i32, i32>::new();
        Self::h(&root, k, &mut m)
    }

    fn h(r: &Option<Rc<RefCell<TreeNode>>>, k: i32, m: &mut HashMap<i32, i32>) -> bool {
        if let Some(r) = r {
            let r = r.borrow();
            if r.val != (k - r.val) {
                m.insert(r.val, (k - r.val)).unwrap_or_default();
            }
            m.get(&(k - r.val)).is_some() || Self::h(&r.left, k, m) || Self::h(&r.right, k, m)
        } else {
            false
        }
    }
}

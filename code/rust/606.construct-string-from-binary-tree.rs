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
    pub fn tree2str(t: Option<Rc<RefCell<TreeNode>>>) -> String {
        Self::h(&t)
    }

    fn h(t: &Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(t) = t {
            let t = t.borrow();
            let l = Self::h(&t.left);
            let r = Self::h(&t.right);
            let mut s = t.val.to_string();
            let f = match (l.is_empty(), r.is_empty()) {
                (true, true) => "".to_string(),
                (false, true) => format!("({})", &l),
                (true, false) => format!("()({})", &r),
                (false, false) => format!("({})({})", &l, &r),
            };
            s += &f;
            s
        } else {
            "".to_string()
        }
    }
}

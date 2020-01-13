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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        Self::h(&root, "", &mut res);
        res
    }

    fn h(t: &Option<Rc<RefCell<TreeNode>>>, mut p: &str, mut r: &mut Vec<String>) {
        if let Some(t) = t {
            let t = t.borrow();
            let val = t.val.to_string();
            let cp = if p.len() == 0 {
                val
            } else {
                p.clone().to_owned() + "->" + &val
            };

            if t.left.is_none() && t.right.is_none() {
                r.push(cp);
            } else {
                Self::h(&t.left, &cp, r);
                Self::h(&t.right, &cp, r);
            }
        }
    }
}

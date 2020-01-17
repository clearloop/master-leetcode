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
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::h(&root1, vec![]) == Self::h(&root2, vec![])
    }

    fn h(r: &Option<Rc<RefCell<TreeNode>>>, mut v: Vec<i32>) -> Vec<i32> {
        if let Some(r) = r {
            let r = r.borrow();
            if r.left.is_none() && r.right.is_none() {
                v.push(r.val);
            }
            Self::h(&r.right, Self::h(&r.left, v))
        } else {
            v
        }
    }
}

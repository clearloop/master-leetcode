// definition for a binary tree node.
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
#![allow(dead_code)]
// mod tree;
use std::cell::RefCell;
use std::rc::Rc;
// use tree::*;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::eq(&p, &q)
    }

    fn eq(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                let (p, q) = (p.borrow(), q.borrow());
                p.val == q.val && Solution::eq(&p.left, &q.left) && Solution::eq(&p.right, &q.right)
            }
            (None, None) => true,
            _ => false,
        }
    }
}

// fn main() {
//     let t = TreeNode::full_from_arr(vec![Some(0), Some(1), Some(2)], 0);
//     let t2 = TreeNode::full_from_arr(vec![Some(0), Some(1), Some(2)], 0);
//     let r = Solution::is_same_tree(t, t2);
//     println!("{:#?}", r);
// }

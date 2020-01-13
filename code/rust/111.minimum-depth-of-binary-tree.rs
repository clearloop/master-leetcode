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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(r) => Self::h(&Some(r)),
            None => 0,
        }
    }

    fn h(r: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = r {
            let lt = Self::h(&r.borrow().left);
            let rt = Self::h(&r.borrow().right);
            if lt == 0 || rt == 0 {
                1 + lt + rt
            } else {
                std::cmp::min(lt, rt) + 1
            }
        } else {
            0
        }
    }
}

// mod tree;
// use tree::*;
//
// fn main() {
//     let tn = TreeNode::full_from_arr(vec![Some(1), None, Some(7)], 0);
//
//     let r = Solution::min_depth(tn);
//     println!("{:#?}", r);
// }

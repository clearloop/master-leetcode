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
    pub fn trim_bst(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        l: i32,
        r: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::h(&mut root, l, r)
    }

    fn h(t: &mut Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(t) = t.take() {
            {
                let mut n = t.borrow_mut();
                if n.val < l {
                    return Self::h(&mut n.right, l, r);
                }
                if n.val > r {
                    return Self::h(&mut n.left, l, r);
                }
                n.left = Self::h(&mut n.left, l, r);
                n.right = Self::h(&mut n.right, l, r);
            }
            Some(t)
        } else {
            None
        }
    }
}

mod tree;
use tree::*;

fn main() {
    let t = TreeNode::full_from_arr(vec![Some(1), Some(0), Some(2)], 0);
    let r = Solution::trim_bst(t, 1, 2);
    println!("{:#?}", r);
}

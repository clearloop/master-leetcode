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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::h(nums)
    }

    pub fn h(v: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if v.len() == 0 {
            return None;
        }

        let mid = v.len() / 2;
        let root = Rc::new(RefCell::new(TreeNode::new(v[mid])));
        root.borrow_mut().left = Self::h(v[..mid].to_vec());
        root.borrow_mut().right = Self::h(v[mid + 1..].to_vec());
        Some(root)
    }
}

// mod tree;
// use tree::*;
//
// fn main() {
//     let bst = Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]);
//     println!("{:#?}", bst);
// }

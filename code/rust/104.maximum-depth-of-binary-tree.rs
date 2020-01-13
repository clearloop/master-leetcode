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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::ruler(&root, 0)
    }

    fn ruler(t: &Option<Rc<RefCell<TreeNode>>>, d: i32) -> i32 {
        if let Some(t) = t {
            let t = t.borrow();
            return std::cmp::max(Self::ruler(&t.left, d + 1), Self::ruler(&t.right, d + 1));
        }

        d
    }
}
// mod tree;
// use tree::*;
//
// pub fn main() {
//     let tn = TreeNode::full_from_arr(
//         vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(17)],
//         0,
//     );
//
//     let r = Solution::max_depth(tn);
//     println!("{:#?}", r);
// }

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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        Self::h(&root, sum)
    }

    fn h(r: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if let Some(r) = r {
            let r = r.borrow();
            let (lt, rt, val) = (&r.left, &r.right, r.val);
            match (lt, rt) {
                (None, None) => sum == val,
                _ => Self::h(&lt, sum - val) || Self::h(&rt, sum - val),
            }
        } else {
            false
        }
    }
}

// mod tree;
// use tree::*;
//
// fn main() {
//     let tn = TreeNode::full_from_arr(
//         vec![
//             Some(5),
//             Some(4),
//             Some(8),
//             Some(11),
//             Some(13),
//             Some(4),
//             Some(7),
//             Some(2),
//             None,
//             Some(1),
//         ],
//         0,
//     );
//
//     let r = Solution::has_path_sum(tn, 22);
//     println!("{:#?}", r);
// }

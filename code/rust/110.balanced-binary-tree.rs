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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::bd(&root).is_ok()
    }

    fn bd(r: &Option<Rc<RefCell<TreeNode>>>) -> Result<i32, ()> {
        if let Some(r) = r {
            let lt = Self::bd(&r.borrow().left)?;
            let rt = Self::bd(&r.borrow().right)?;
            if (lt - rt).abs() > 1 {
                Err(())
            } else {
                Ok(1 + std::cmp::max(lt, rt))
            }
        } else {
            Ok(0)
        }
    }
}

// mod tree;
// use tree::*;
//
// fn main() {
//     let tn = TreeNode::full_from_arr(
//         vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
//         0,
//     );
//
//     let r = Solution::is_balanced(tn);
//     println!("{:#?}", r);
// }

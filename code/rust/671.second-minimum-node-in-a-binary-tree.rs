// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {j
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
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut vals: Vec<i32> = vec![];
        Self::h(&root, &mut vals);
        vals.sort();
        vals.dedup();
        if vals.len() >= 2 {
            vals[1]
        } else {
            -1
        }
    }

    fn h(r: &Option<Rc<RefCell<TreeNode>>>, mut vals: &mut Vec<i32>) {
        if let Some(r) = r {
            let r = r.borrow();
            vals.push(r.val);
            Self::h(&r.left, &mut vals);
            Self::h(&r.right, &mut vals);
        }
    }
}

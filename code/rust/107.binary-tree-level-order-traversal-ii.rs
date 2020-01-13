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
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        Self::bu(0, &root, res.as_mut());
        res.reverse();
        res
    }

    fn bu(d: usize, r: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<Vec<i32>>) {
        if let Some(r) = r {
            if res.len() == d {
                res.push(Vec::new());
            }

            res[d].push(r.borrow().val);
            Self::bu(d + 1, &r.borrow().left, res);
            Self::bu(d + 1, &r.borrow().right, res);
        }
    }
}

// mod tree;
// use tree::*;
//
// fn main() {
//     let tn = TreeNode::full_from_arr(
//         vec![Some(1), Some(2), Some(3), Some(4), None, None, Some(5)],
//         0,
//     );
//
//     let r = Solution::level_order_bottom(tn);
//     println!("{:#?}", r);
// }

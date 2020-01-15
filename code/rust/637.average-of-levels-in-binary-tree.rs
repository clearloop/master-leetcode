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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut res: Vec<f64> = vec![];
        let mut pre_res: Vec<Vec<f64>> = vec![];
        Self::h(&root, &mut pre_res, 0);
        pre_res.iter().for_each(|a| {
            res.push(a.iter().sum::<f64>() / a.len() as f64);
        });
        res
    }

    fn h(r: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<Vec<f64>>, level: usize) {
        if let Some(r) = r {
            let r = r.borrow();
            if res.len() < level + 1 {
                res.push(vec![r.val as f64]);
            } else {
                res[level].push(r.val as f64);
            }

            Self::h(&r.left, res, level + 1);
            Self::h(&r.right, res, level + 1);
        }
    }
}

// mod tree;
// use tree::*;
//
// fn main() {
//     let l = TreeNode::full_from_arr(
//         vec![
//             Some(3),
//             Some(1),
//             Some(5),
//             Some(0),
//             Some(2),
//             Some(4),
//             Some(6),
//         ],
//         0,
//     );
//     let r = Solution::average_of_levels(l);
//     println!("{:#?}", r);
// }

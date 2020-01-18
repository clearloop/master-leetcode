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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let mut m: HashMap<i32, Vec<Option<i32>>> = HashMap::new();
        Self::h(&root, &mut m, 0);

        let mut res = false;
        m.values().for_each(|i| {
            let mut xi = -1;
            let mut yi = -1;
            i.iter().enumerate().for_each(|(p, q)| {
                if q == &Some(x) {
                    xi = p as i32;
                } else if q == &Some(y) {
                    yi = p as i32;
                }

                if (xi != -1) && (yi != -1) {
                    if (xi - yi).abs() == 1 {
                        if xi.max(yi) % 2 == 0 {
                            res = true;
                        }
                    } else {
                        res = true;
                    }
                }
            });
        });

        res
    }

    fn h(r: &Option<Rc<RefCell<TreeNode>>>, m: &mut HashMap<i32, Vec<Option<i32>>>, d: i32) {
        if let Some(r) = r {
            let r = r.borrow();
            if let Some(v) = m.get_mut(&d) {
                v.push(Some(r.val));
            } else {
                m.insert(d, vec![Some(r.val)]);
            }
            Self::h(&r.left, m, d + 1);
            Self::h(&r.right, m, d + 1);
        } else {
            if let Some(v) = m.get_mut(&d) {
                v.push(None);
            } else {
                m.insert(d, vec![None]);
            }
        }
    }
}

// mod tree;
// use tree::*;
//
// fn main() {
//     let t = TreeNode::full_from_arr(
//         // vec![Some(1), Some(2), Some(3), None, Some(4), None, Some(5)],
//         vec![Some(1), Some(2), Some(3), Some(4)],
//         0,
//     );
//     let r = Solution::is_cousins(t, 3, 4);
//     println!("{:#?}", r);
// }

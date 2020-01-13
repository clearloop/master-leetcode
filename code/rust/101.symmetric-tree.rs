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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(r) = root {
            let r = r.borrow();
            return Self::mirror(&r.left, &r.right);
        }

        true
    }

    pub fn mirror(t1: &Option<Rc<RefCell<TreeNode>>>, t2: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (t1, t2) {
            (&None, &None) => true,
            (Some(n1), Some(n2)) => {
                let (n1, n2) = (n1.borrow(), n2.borrow());

                n1.val == n2.val
                    && Self::mirror(&n1.left, &n2.right)
                    && Self::mirror(&n2.left, &n1.right)
            }
            _ => false,
        }
    }
}

// mod tree;
// use tree::*;
// fn main() {
//     let t = TreeNode::full_from_arr(
//         vec![
//             Some(1),
//             Some(2),
//             Some(2),
//             Some(3),
//             Some(4),
//             Some(4),
//             Some(3),
//         ],
//         0,
//     );
//
//     let r = Solution::is_symmetric(t);
//     println!("{:?}", r);
// }

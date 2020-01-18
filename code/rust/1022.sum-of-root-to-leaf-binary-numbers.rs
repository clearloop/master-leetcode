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
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        Self::h(&root, &mut res, &mut vec![]);
        res
    }

    fn h(t: &Option<Rc<RefCell<TreeNode>>>, res: &mut i32, p: &mut Vec<i32>) {
        if let Some(t) = t {
            let t = t.borrow();
            p.push(t.val);
            if t.left.is_none() && t.right.is_none() {
                let mut c = 0;
                p.iter().for_each(|x| c = (c << 1) | x);
                *res += c;
            } else {
                Self::h(&t.left, res, &mut p.clone());
                Self::h(&t.right, res, &mut p.clone());
            }
        }
    }
}

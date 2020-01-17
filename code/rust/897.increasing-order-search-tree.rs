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
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut vals: Vec<i32> = Vec::new();
        let dim = Rc::new(RefCell::new(TreeNode::new(-1)));
        Self::h(&root, &mut vals);
        // empty vals
        if vals.is_empty() {
            return None;
        }

        // share ownership after clone.
        let mut cur = dim.clone();
        vals.iter().for_each(|i| {
            let n = Rc::new(RefCell::new(TreeNode::new(*i)));
            cur.borrow_mut().right = Some(n.clone());
            cur = n;
        });

        let tmp = &dim.borrow().right;
        tmp.to_owned()
    }

    fn h(r: &Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
        if let Some(r) = r {
            let r = r.borrow();
            if r.left.is_some() {
                Self::h(&r.left, vals);
            }

            vals.push(r.val);

            if r.right.is_some() {
                Self::h(&r.right, vals);
            }
        }
    }
}

// mod tree;
// use tree::*;
//
// fn main() {
//     let t = TreeNode::bst_from_arr(vec![Some(1), Some(2), Some(3)]);
//     let r = Solution::increasing_bst(t);
//     println!("{:#?}", r);
// }

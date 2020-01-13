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
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut counter: HashMap<i32, i32> = HashMap::new();
        let mut rth = vec![];
        Self::h(&root, &mut counter);

        // !important
        if counter.is_empty() {
            return rth;
        }

        let max = *counter.values().max().unwrap();
        for (&k, &v) in counter.iter() {
            if v == max {
                rth.push(k);
            }
        }
        rth
    }

    fn h(r: &Option<Rc<RefCell<TreeNode>>>, counter: &mut HashMap<i32, i32>) {
        if let Some(r) = r {
            let r = r.borrow();
            let count = counter.entry(r.val).or_insert(0);
            *count += 1;
            Self::h(&r.left, counter);
            Self::h(&r.right, counter);
        }
    }
}

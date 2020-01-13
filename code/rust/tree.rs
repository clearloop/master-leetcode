use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn bst_from_arr(arr: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if arr.len() == 0 {
            return None;
        }

        let mid = arr.len() / 2;
        let root = Rc::new(RefCell::new(TreeNode::new(arr[mid].unwrap())));
        root.borrow_mut().left = Self::bst_from_arr(arr[..mid].to_vec());
        root.borrow_mut().right = Self::bst_from_arr(arr[mid + 1..].to_vec());

        Some(root)
    }

    pub fn full_from_arr(arr: Vec<Option<i32>>, i: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if arr.len() == 0 || i >= arr.len() || arr[i] == None {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(arr[i].unwrap())));
        root.borrow_mut().left = Self::full_from_arr(arr.clone(), 2 * i + 1);
        root.borrow_mut().right = Self::full_from_arr(arr.clone(), 2 * i + 2);

        Some(root)
    }
}

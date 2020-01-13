use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn lca(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || p.is_none() || q.is_none() {
            return None;
        }

        // lca should not be none
        let (mut root, p, q) = (root.unwrap(), p.unwrap(), q.unwrap());
        while (root.val - p.val) * (root.val - q.val) > 0 {
            if root == None {
                return None;
            }

            if p.val > root.val {
                root = root.right.unwrap_or(None);
            } else {
                root = root.left.unwrap_or(None);
            }
        }
        root
    }
}

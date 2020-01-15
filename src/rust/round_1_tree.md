# Tree

## 100.same-tree

```rust
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::eq(&p, &q)
    }

    fn eq(
	  p: &Option<Rc<RefCell<TreeNode>>>, 
	  q: &Option<Rc<RefCell<TreeNode>>>
	) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                let (p, q) = (p.borrow(), q.borrow());
                p.val == q.val 
				  && Solution::eq(&p.left, &q.left) 
				  && Solution::eq(&p.right, &q.right)
            }
            (None, None) => true,
            _ => false,
        }
    }
}
```


## 101.symmetric-tree 

```rust
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

    pub fn mirror(
	  t1: &Option<Rc<RefCell<TreeNode>>>, 
	  t2: &Option<Rc<RefCell<TreeNode>>>
	) -> bool {
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
```

## 104.maximum-depth-of-binary-tree
```rust
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::ruler(&root, 0)
    }

    fn ruler(t: &Option<Rc<RefCell<TreeNode>>>, d: i32) -> i32 {
        if let Some(t) = t {
            let t = t.borrow();
            return std::cmp::max(
			  Self::ruler(&t.left, d + 1), 
			  Self::ruler(&t.right, d + 1)
		    );
        }

        d
    }
}
```

## 107.binary-tree-level-order-traversal-ii
```rust
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
```

## 108.convert-sorted-array-to-binary-earch-tree

```rust
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::h(nums)
    }

    pub fn h(v: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if v.len() == 0 {
            return None;
        }

        let mid = v.len() / 2;
        let root = Rc::new(RefCell::new(TreeNode::new(v[mid])));
        root.borrow_mut().left = Self::h(v[..mid].to_vec());
        root.borrow_mut().right = Self::h(v[mid + 1..].to_vec());
        Some(root)
    }
}
```


## 110.balanced-binary-tree
```rust
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::bd(&root).is_ok()
    }

    fn bd(r: &Option<Rc<RefCell<TreeNode>>>) -> Result<i32, ()> {
        if let Some(r) = r {
            let lt = Self::bd(&r.borrow().left)?;
            let rt = Self::bd(&r.borrow().right)?;
            if (lt - rt).abs() > 1 {
                Err(())
            } else {
                Ok(1 + std::cmp::max(lt, rt))
            }
        } else {
            Ok(0)
        }
    }
}
```

## 111.minmum-depth-of-binary-tree

```rust
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(r) => Self::h(&Some(r)),
            None => 0,
        }
    }

    fn h(r: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = r {
            let lt = Self::h(&r.borrow().left);
            let rt = Self::h(&r.borrow().right);
            if lt == 0 || rt == 0 {
                1 + lt + rt
            } else {
                std::cmp::min(lt, rt) + 1
            }
        } else {
            0
        }
    }
}
```

## 112.path-sum

```rust
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        Self::h(&root, sum)
    }

    fn h(r: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if let Some(r) = r {
            let r = r.borrow();
            let (lt, rt, val) = (&r.left, &r.right, r.val);
            match (lt, rt) {
                (None, None) => sum == val,
                _ => Self::h(&lt, sum - val) || Self::h(&rt, sum - val),
            }
        } else {
            false
        }
    }
}
```

## 226.invert-binary-tree
```rust
impl Solution {
    pub fn invert_tree(
	  mut root: Option<Rc<RefCell<TreeNode>>>
	) -> Option<Rc<RefCell<TreeNode>>> {
        Self::h(&mut root);
        root
    }

    fn h(r: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(r) = r {
            let mut r = r.borrow_mut();
            Self::h(&mut r.left);
            Self::h(&mut r.right);

            let lt = std::mem::replace(&mut r.left, None);
            let rt = std::mem::replace(&mut r.right, lt);
            std::mem::replace(&mut r.left, rt);
        }
    }
}
```

## 235.lowest-common-ancestor-of-a-binary-search-tree
```rust
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
```

## 257.binary-tree-paths
```rust
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        Self::h(&root, "", &mut res);
        res
    }

    fn h(t: &Option<Rc<RefCell<TreeNode>>>, mut p: &str, mut r: &mut Vec<String>) {
        if let Some(t) = t {
            let t = t.borrow();
            let val = t.val.to_string();
            let cp = if p.len() == 0 {
                val
            } else {
                p.clone().to_owned() + "->" + &val
            };

            if t.left.is_none() && t.right.is_none() {
                r.push(cp);
            } else {
                Self::h(&t.left, &cp, r);
                Self::h(&t.right, &cp, r);
            }
        }
    }
}
```

## 404.sum-of-left-leaves
```rust
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        Self::h(&root, &mut sum);
        sum
    }

    fn h(r: &Option<Rc<RefCell<TreeNode>>>, s: &mut i32) {
        if let Some(r) = r {
            let r = r.borrow();

            // add
            if let Some(l) = &r.left {
                let l = l.borrow();
                if l.left.is_none() && l.right.is_none() {
                    *s += l.val;
                }
            }

            // rescursion
            Self::h(&r.left, s);
            Self::h(&r.right, s);
        }
    }
}
```

## 437.path-sum-iii

```rust
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut res = 0;
        Self::h(&mut res, sum, &root);
        if let Some(r) = root {
            let r = r.borrow();
            res += Self::path_sum(r.left.clone(), sum);
            res += Self::path_sum(r.right.clone(), sum);
        }
        res
    }

    fn h(r: &mut i32, s: i32, t: &Option<Rc<RefCell<TreeNode>>>) {
        if let Some(t) = t {
            let t = t.borrow();
            if t.val == s {
                *r += 1;
            }

            Self::h(r, s - t.val, &t.left);
            Self::h(r, s - t.val, &t.right);
        }
    }
}
```

## 501.find-mode-in-binary-search-tree
```rust
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
```

## 530.minimum-absolute-difference-in-bst
```rust
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::h(&root, MIN, MAX).1
    }

    fn h(r: &Option<Rc<RefCell<TreeNode>>>, last: i32, res: i32) -> (i32, i32) {
        if let Some(r) = r {
            let r = r.borrow();
            let (mut last, mut res) = Self::h(&r.left.clone(), last, res);
            res = res.min(r.val.saturating_sub(last));
            last = r.val;
            return Self::h(&r.right.clone(), last, res);
        }
        return (last, res);
    }
}
```

## 538.convert-bst-to-greater-tree

```rust
impl Solution {
    pub fn convert_bst(
	  mut root: Option<Rc<RefCell<TreeNode>>>
	) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum = 0;
        Self::h(&mut root, &mut sum);
        root
    }

    fn h(r: &mut Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if let Some(r) = r {
            let mut r = r.borrow_mut();
            Self::h(&mut r.right, sum);
            r.val += *sum;
            *sum = r.val;
            Self::h(&mut r.left, sum);
        }
    }
}
```

## 543.diameter-of-binary-tree
```rust
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut d = 1;
        Self::h(&root, &mut d);
        d - 1
    }

    fn h(r: &Option<Rc<RefCell<TreeNode>>>, d: &mut i32) -> i32 {
        if let Some(r) = r {
            let r = r.borrow();
            let p = Self::h(&r.left, d);
            let q = Self::h(&r.right, d);
            *d = d.to_owned().max(p + q + 1);
            1 + p.max(q)
        } else {
            0
        }
    }
}
```

## 563.binary-tree-tilt
```rust
impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut s = 0;
        Self::h(&root, &mut s);
        s
    }

    fn h(n: &Option<Rc<RefCell<TreeNode>>>, s: &mut i32) -> i32 {
        if let Some(n) = n {
            let n = n.borrow();
            let l = Self::h(&n.left, s);
            let r = Self::h(&n.right, s);
            *s += (l - r).abs();
            n.val + l + r
        } else {
            0
        }
    }
}
```

## 572.subtree-of-another-tree
```rust
impl Solution {
    pub fn is_subtree(
	  s: Option<Rc<RefCell<TreeNode>>>, 
	  t: Option<Rc<RefCell<TreeNode>>>,
	) -> bool {
        Self::h(&s, &t)
    }

    fn h(
	  s: &Option<Rc<RefCell<TreeNode>>>, 
	  t: &Option<Rc<RefCell<TreeNode>>>
	) -> bool {
        if s == t {
            return true;
        } else if s.is_none() {
            return false;
        }

        let s = s.as_ref().unwrap().borrow();
        Self::h(&s.left, t) || Self::h(&s.right, t)
    }
}
```

## 606.construct-string-from-binary-tree
```rust
impl Solution {
    pub fn tree2str(t: Option<Rc<RefCell<TreeNode>>>) -> String {
        Self::h(&t)
    }

    fn h(t: &Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(t) = t {
            let t = t.borrow();
            let l = Self::h(&t.left);
            let r = Self::h(&t.right);
            let mut s = t.val.to_string();
            let f = match (l.is_empty(), r.is_empty()) {
                (true, true) => "".to_string(),
                (false, true) => format!("({})", &l),
                (true, false) => format!("()({})", &r),
                (false, false) => format!("({})({})", &l, &r),
            };
            s += &f;
            s
        } else {
            "".to_string()
        }
    }
}
```

## 617.merge-two-binary-trees
```rust
impl Solution {
    pub fn merge_trees(
        mut t1: Option<Rc<RefCell<TreeNode>>>,
        mut t2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::h(&mut t1, &mut t2);
        t1
    }

    fn h(
	  p: &mut Option<Rc<RefCell<TreeNode>>>, 
	  q: &mut Option<Rc<RefCell<TreeNode>>>,
	) {
        if p.is_some() && q.is_some() {
            let (mut p, mut q) = (
                p.as_mut().unwrap().borrow_mut(),
                q.as_mut().unwrap().borrow_mut(),
            );
            p.val += q.val;
            Self::h(&mut p.left, &mut q.left);
            Self::h(&mut p.right, &mut q.right);
        } else if p.is_none() && q.is_some() {
            *p = q.take();
        }
    }
}
```

## 637.average-of-levels-in-binary-tree
```rust
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
```

## 653.two-sum-iv-input-is-a-bst

```rust
impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut m = HashMap::<i32, i32>::new();
        Self::h(&root, k, &mut m)
    }

    fn h(
	  r: &Option<Rc<RefCell<TreeNode>>>, 
	  k: i32, m: &mut HashMap<i32, i32>
	) -> bool {
        if let Some(r) = r {
            let r = r.borrow();
            if r.val != (k - r.val) {
                m.insert(r.val, (k - r.val)).unwrap_or_default();
            }
            m.get(&(k - r.val)).is_some() 
			  || Self::h(&r.left, k, m) 
			  || Self::h(&r.right, k, m)
        } else {
            false
        }
    }
}
```

## 669.trim-a-binary-search-tree
```rust
impl Solution {
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        l: i32,
        r: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::h(&root, l, r)
    }

    fn h(
	  t: &Option<Rc<RefCell<TreeNode>>>, 
	  l: i32, 
	  r: i32
	) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(t) = t {
            let mut n = t.borrow_mut();
            if n.val < l {
                return Self::h(&n.right, l, r);
            }
            if n.val > r {
                return Self::h(&n.left, l, r);
            }
            n.left = Self::h(&n.left, l, r);
            n.right = Self::h(&n.right, l, r);
            Some(t.clone())
        } else {
            None
        }
    }
}
```

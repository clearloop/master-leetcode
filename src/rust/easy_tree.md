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

## 671.second-minimum-node-in-a-binary-tree
```rust
impl Solution {
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut vals: Vec<i32> = vec![];
        Self::h(&root, &mut vals);
        vals.sort();
        vals.dedup();
        if vals.len() >= 2 {
            vals[1]
        } else {
            -1
        }
    }

    fn h(r: &Option<Rc<RefCell<TreeNode>>>, mut vals: &mut Vec<i32>) {
        if let Some(r) = r {
            let r = r.borrow();
            vals.push(r.val);
            Self::h(&r.left, &mut vals);
            Self::h(&r.right, &mut vals);
        }
    }
}
```

## 687.longest-univablue-path ⭐️
```rust
impl Solution {
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut c = 0;
        Self::h(&root, &mut c);
        c
    }

    fn h(r: &Option<Rc<RefCell<TreeNode>>>, c: &mut i32) -> i32 {
        if let Some(r) = r {
            let r = r.borrow();
            let mut tlc = Self::h(&r.left, c);
            let mut trc = Self::h(&r.right, c);

            let (mut lc, mut rc) = (0, 0);
            if let Some(tl) = &r.left {
                let tl = tl.borrow();
                if tl.val == r.val {
                    lc = tlc + 1;
                }
            }

            if let Some(tr) = &r.right {
                let tr = tr.borrow();
                if tr.val == r.val {
                    rc = trc + 1;
                }
            }

            *c = *c.max(&mut (lc + rc));
            lc.max(rc)
        } else {
            0
        }
    }
}
```

## 700.search-in-a-binary-search-tree
```rust
impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::h(&root, val)
    }

    fn h(
	  r: &Option<Rc<RefCell<TreeNode>>>, 
	  val: i32
	) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(rt) = r {
            let rt = rt.borrow();
            if rt.val == val {
                r.to_owned()
            } else if rt.val > val {
                Self::h(&rt.left, val)
            } else {
                Self::h(&rt.right, val)
            }
        } else {
            None
        }
    }
}
```

## 783.minimum-distance-between-bst-node ⭐️
```rust
impl Solution {
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min = i32::MAX;
        let mut pre = i32::MIN;
        Self::h(&root, &mut pre, &mut min);
        min
    }

    fn h(r: &Option<Rc<RefCell<TreeNode>>>, pre: &mut i32, min: &mut i32) {
        if let Some(r) = r {
            let r = r.borrow();
            Self::h(&r.left, pre, min);
            *min = (r.val.saturating_sub(*pre)).min(*min);
            *pre = r.val; // !important;
            Self::h(&r.right, pre, min);
        }
    }
}
```

## 872.leaf-similar-trees
```rust
impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::h(&root1, vec![]) == Self::h(&root2, vec![])
    }

    fn h(r: &Option<Rc<RefCell<TreeNode>>>, mut v: Vec<i32>) -> Vec<i32> {
        if let Some(r) = r {
            let r = r.borrow();
            if r.left.is_none() && r.right.is_none() {
                v.push(r.val);
            }
            Self::h(&r.right, Self::h(&r.left, v))
        } else {
            v
        }
    }
}
```

## 897.increasing-order-search-tree ⭐️
```rust
impl Solution {
    pub fn increasing_bst(
	  root: Option<Rc<RefCell<TreeNode>>>
	) -> Option<Rc<RefCell<TreeNode>>> {
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
```


## 938.range-sum-of-bst

```rust
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
        let mut sum = 0;
        Self::h(&root, &mut sum, l, r);
        sum
    }

    fn h(t: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32, l: i32, r: i32) {
        if let Some(t) = t {
            let t = t.borrow();
            if t.val <= r && t.val >= l {
                *sum += t.val;
            }

            Self::h(&t.left, sum, l, r);
            Self::h(&t.right, sum, l, r);
        }
    }
}
```

## 965.univalued-binary-tree

```rust
impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut res = true;
        Self::h(&root, &mut res);
        res
    }

    fn h(t: &Option<Rc<RefCell<TreeNode>>>, res: &mut bool) {
        if let Some(t) = t {
            let t = t.borrow();
            if let Some(l) = &t.left {
                if l.borrow().val != t.val {
                    *res = false;
                }
                Self::h(&t.left, res);
            }

            if let Some(r) = &t.right {
                if r.borrow().val != t.val {
                    *res = false;
                }
                Self::h(&t.right, res);
            }
        }
    }
}
```

## 993.cusins-in-binary-tree
```rust
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
            })
        });

        res
    }

    fn h(
	  r: &Option<Rc<RefCell<TreeNode>>>, 
	  m: &mut HashMap<i32, Vec<Option<i32>>>, 
	  d: i32
	) {
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
```

## 1022.sum-of-root-to-leaf-binary-numbers
```rust
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
```

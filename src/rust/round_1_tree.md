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

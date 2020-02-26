# lcof

## 5.replace-space
```rust
impl Solution {
    pub fn replace_space(s: String) -> String {
        let mut res = String::new();
        s.chars().for_each(|x| if x.is_whitespace() {
            res.push_str("%20");
        } else {
            res.push(x);
        });
        
        res
    }
}
```

## 17.print-n-numbers-from-1
```rust
impl Solution {
    pub fn print_numbers(n: i32) -> Vec<i32> {
        (1..("9".repeat(n as usize).parse().unwrap_or(1)) + 1).collect()
    }
}
```

## 22.k-node-in-reverse-linked-list
```rust
impl Solution {
    pub fn get_kth_from_end(
      mut head: Option<Box<ListNode>>, k: i32
    ) -> Option<Box<ListNode>> {
        let mut ptr = head.clone();
        for i in 0..k {
            if let Some(ln) = ptr {
                ptr = ln.next;
            } else {
                return None;
            }
        }

        while ptr.is_some() {
            ptr = ptr.unwrap().next;
            head = head.unwrap().next;
        }

        head
    }
}
```

## 27.invert-binary-tree

```rust
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn mirror_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::h(&mut root);
        root
    }

    pub fn h(r: &mut Option<Rc<RefCell<TreeNode>>>) {
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

## 56.repeat-n-numbers-in-array
```rust
impl Solution {
    pub fn single_number(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        for (idx, n) in nums.iter().enumerate() {
            if n != nums.get(idx - 1).unwrap_or(&0) && n != nums.get(idx + 1).unwrap_or(&0) {
                return *n;
            }
        }

        0
    }
}
```

## 58.reverse-left-words
```rust
impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        let mut res = String::new();
        res.push_str(&s[(n as usize)..]);
        res.push_str(&s[..(n as usize)]);
        res
    }
}
```

## 64.sum-nums
```rust
impl Solution {
    pub fn sum_nums(n: i32) -> i32 {
        (1 + n) * n / 2
    }
}
```

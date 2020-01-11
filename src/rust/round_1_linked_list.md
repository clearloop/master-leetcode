# Linked-List

## 21.merge-two-sorted-lists

```rust
impl Solution {
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        use std::mem;
		
        let mut pa = &mut l1;
        let pb = &mut l2;

        // Merge l2 to l1
        //
        // Current List ->  "**>>>"
        // Dynamic List ->  ">>>>>"
        while pb.is_some() {
            if pa.is_none() || pa.as_ref()?.val > pb.as_ref()?.val {
                mem::swap(pa, pb);
            } else {
                pa = &mut pa.as_mut()?.next;
            }
        }

        l1
    }
}
```

## 83.remove-duplicate-from-sorted-list
```rust
impl Solution {
    pub fn delete_duplicates(
	  mut head: Option<Box<ListNode>>
	) -> Option<Box<ListNode>> {
        let mut ptr = head.as_mut()?;

        while let Some(next) = ptr.next.as_mut() {
            if ptr.val == next.val {
                // let tmp = std::mem::replace(&mut next.next, None);
                // std::mem::replace(&mut ptr.next, tmp);
                ptr.next = next.next.take();
            } else {
                ptr = ptr.next.as_mut()?;
            }
        }

        head
    }
}
```

## 141. linked-list-cycle

```rust
impl Solution {
    pub fn has_circle(mut head: Option<Box<ListNode>>) -> Option<bool> {
        let mut slow = &mut head;
        let mut fast = slow.clone();

        while slow.is_some() && slow.as_ref()?.next.as_ref()?.next.is_some() {
            slow = &mut slow.as_mut()?.next;
            fast = fast.unwrap().next.unwrap().next;
			
            // if two pointers meet, cycle exists.
            if slow.as_ref()? == fast.as_ref()? {
                return Some(true);
            }
        }

        return None;
    }
}
```

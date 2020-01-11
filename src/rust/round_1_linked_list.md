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
        let mut ptr = &mut head;
        let mut _nxt: Option<Box<ListNode>> = None;

        while ptr.is_some() && ptr.as_ref()?.next.is_some() {
            if ptr.as_ref()?.val == ptr.as_ref()?.next.as_ref()?.val {
                // make sure not using next.next directly
                _nxt = ptr.as_mut()?.next.take();
                ptr.as_mut()?.next = _nxt.as_mut()?.next.take();
            }
            ptr = &mut ptr.as_mut()?.next;
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

## 160. intersection-of-two-linked-list
```rust
impl Solution {
    pub fn get_intersection(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut pa = &mut l1.clone();
        let mut pb = &mut l2.clone();

        while pa.is_some() && pb.is_some() {
		    // The end of both two is the intersection.
            if pa == pb {
                return pa.to_owned();
            }

            if pa.as_ref()?.next.is_some() {
                pa = &mut pa.as_mut()?.next;
            } else {
                pa = &mut l2;
            }

            if pb.as_ref()?.next.is_some() {
                pb = &mut pb.as_mut()?.next;
            } else {
                pb = &mut l1;
            }
        }

        None
    }
}
```

## 203 remove-linked-list-element
```rust
impl Solution {
    pub fn remove_elements(
	  mut head: Option<Box<ListNode>>, val: i32
	) -> Option<Box<ListNode>> {
        let mut ptr = &mut head;

        while ptr.is_some() {
            // replace current value in memory
            if ptr.as_ref()?.val == val {
                // *ptr = std::mem::replace(&mut ptr.as_mut()?.next, None);
                *ptr = ptr.as_mut()?.next.take();
            } else {
                ptr = &mut ptr.as_mut()?.next;
            }
        }

        head
    }
}
```

## 206. reverse-linked-list
```rust
impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ret = None;
        let mut _nxt = None;

        while head.is_some() {
            _nxt = head.as_mut()?.next.take();
            head.as_mut()?.next = ret;
            ret = head;

            // Reverse
            //
            // Old ListNode: "***>>"
            // Ret ListNode: ">>>"
            head = _nxt;
        }

        ret
    }
}
```

## 234. palindrome-linked-list
```rust
impl Solution {
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return true;
        }

        let mut rev: Option<Box<ListNode>> = None;
        let mut _nxt: Option<Box<ListNode>> = None;
        
		// Just like 206, half to half
        while head.is_some() {
            if rev == head || rev == head.as_ref().unwrap().next {
                return true;
            }

            _nxt = head.as_mut().unwrap().next.take();
            head.as_mut().unwrap().next = rev;
            rev = head;
            head = _nxt;
        }

        false
    }
}
```

## 237.delete-node-in-a-linked-list

> Same as 203

## 876.middle-of-the-linked-list

```rust
impl Solution {
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ptr = head.clone();

        while ptr.is_some() && ptr.as_ref()?.next.is_some() {
            ptr = ptr.unwrap().next.unwrap().next;
            head = head.unwrap().next;
        }

        head
    }
}
```

## 1290.convert-binary-number-in-a-linked-list-to-integer

```rust
impl Solution {
    pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
        let mut ret = 0;
        while head.is_some() {
            ret = (ret << 1) | head.as_ref().unwrap().val;
            head = head.unwrap().next;
        }

        ret
    }
}
```

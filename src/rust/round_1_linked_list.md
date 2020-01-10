# Linked-List

## 21.merge-two-sorted-lists

```rust
# struct Solution;
# #[derive(PartialEq, Eq, Clone, Debug)]
# pub struct ListNode {
#     pub val: i32,
#     pub next: Option<Box<ListNode>>,
# }
#
# impl ListNode {
#     #[inline]
#     fn new(val: i32) -> Self {
#         ListNode { next: None, val }
#     }
# }
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

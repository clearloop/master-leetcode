#[allow(dead_code)]
pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from(l: Vec<i32>) -> Option<Box<ListNode>> {
        let mut node = ListNode::new(0);
        let mut ptr = &mut node;

        for i in l {
            ptr.next = Some(Box::new(ListNode::new(i)));
            ptr = ptr.next.as_mut()?;
        }

        node.next
    }
}

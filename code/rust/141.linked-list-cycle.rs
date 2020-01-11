// mod ll;
// use ll::*;
impl Solution {
    pub fn has_circle(mut head: Option<Box<ListNode>>) -> Option<bool> {
        let mut slow = &mut head;
        let mut fast = slow.clone();

        while slow.is_some() && slow.as_ref()?.next.as_ref()?.next.is_some() {
            slow = &mut slow.as_mut()?.next;
            fast = fast.unwrap().next.unwrap().next;

            if slow.as_ref()? == fast.as_ref()? {
                return Some(true);
            }
        }

        return None;
    }
}

// fn main() {
//     let ln = ListNode::from(vec![1, 2, 3, 5, 3, 5]);
//     let r = Solution::has_circle(ln);
//     println!("{:?}", r);
// }

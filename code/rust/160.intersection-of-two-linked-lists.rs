mod ll;
use ll::*;

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

fn main() {
    let l1 = ListNode::from(vec![1, 2, 3]);
    let l2 = ListNode::from(vec![1, 1, 1, 2, 3]);

    let r = Solution::get_intersection(l1, l2);
    println!("{:#?}", r);
}

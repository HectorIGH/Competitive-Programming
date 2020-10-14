//Given the head of a linked list, return the list after sorting it in ascending order.
//
//Follow up: Can you sort the linked list in O(n logn) time and O(1) memory (i.e. constant space)?
//
// 
//
//Example 1:
//
//
//Input: head = [4,2,1,3]
//Output: [1,2,3,4]
//Example 2:
//
//
//Input: head = [-1,5,3,4,0]
//Output: [-1,0,3,4,5]
//Example 3:
//
//Input: head = []
//Output: []
// 
//
//Constraints:
//
//The number of nodes in the list is in the range [0, 5 * 104].
//-105 <= Node.val <= 105

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (lhs, rhs) = Solution::split(head);
        match (lhs, rhs) {
            (None, None) => None,
            (lhs, None) => lhs,
            (None, rhs) => rhs,
            (lhs, rhs) => Solution::merge(Solution::sort_list(lhs), Solution::sort_list(rhs))
        }
    }
    pub fn merge(lhs: Option<Box<ListNode>>, rhs: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (lhs, rhs) {
            (None, None) => None,
            (node, None) => node,
            (None, node) => node,
            (Some(mut h), Some(mut t)) => {
                let (mut small, large) = if h.val < t.val {
                    (h, t)
                } else {
                    (t, h)
                };
                let successor = small.next.take();
                small.next = Solution::merge(successor, Some(large));
                Some(small)
            }
        }
    }

    pub fn split(mut head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut lhs = None;
        let mut rhs = None;
        let mut i: bool = true;
        loop {
            head = match head {
                None => break,
                Some(mut headnode) => {
                    let head = headnode.next.take();
                    match i {
                        true => {
                            headnode.next = lhs.take();
                            lhs = Some(headnode);
                        },
                        false => {
                            headnode.next = rhs.take();
                            rhs = Some(headnode);
                        }
                    }
                    head
                }
            };
            i = !i;
        }
        (lhs, rhs)
    }
}
//Given a linked list, rotate the list to the right by k places, where k is non-negative.
//
//Example 1:
//
//Input: 1->2->3->4->5->NULL, k = 2
//Output: 4->5->1->2->3->NULL
//Explanation:
//rotate 1 steps to the right: 5->1->2->3->4->NULL
//rotate 2 steps to the right: 4->5->1->2->3->NULL
//Example 2:
//
//Input: 0->1->2->NULL, k = 4
//Output: 2->0->1->NULL
//Explanation:
//rotate 1 steps to the right: 2->0->1->NULL
//rotate 2 steps to the right: 1->2->0->NULL
//rotate 3 steps to the right: 0->1->2->NULL
//rotate 4 steps to the right: 2->0->1->NULL

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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
        /*if head.is_none() || k == 0 {
            return head;
        }
        let mut size: i32 = 0;
        { // Creating scope so we can borrow head again
            let mut last = &mut head;
            while last.is_some() {
                size += 1;
                last = &mut last.as_mut().unwrap().next;
            }
        }
        if size <= 1 || k % size == 0 {
            return head;
        }
        k = if k < size {k} else {k % size};
        let mut new_last = &mut head;
        for _ in 0..(size - k - 1) {
            new_last = &mut new_last.as_mut().unwrap().next;
        }
        let mut new_head = new_last.as_mut().unwrap().next.take();
        new_last.as_mut().unwrap().next = None;
        
        new_last = &mut new_head;
        
        while new_last.as_mut().unwrap().next.is_some() {
            new_last = &mut new_last.as_mut().unwrap().next;
        }
        new_last.as_mut().unwrap().next = head;
        return new_head;*/
        
        if head.is_none() || k == 0 {
            return head;
        }
        let mut l: Vec<i32> = Vec::new();
        let mut pointer = &head;
        while let Some(n) = pointer {
            l.push(n.val);
            pointer = &n.next;
        }
        let mut K: usize = k as usize;
        K = if K < l.len() {K} else {K % l.len()};
        // Using built-in rotate function in Vec crate
        l.rotate_right(K);
        // Can be rotated using slices. Less memory
        //let s = l.len();
        //let mut ll = l[(s - K)..].to_vec();
        //ll.append(&mut l[..(s - K)].to_vec());
        //l = ll;
        // Construct in reverse order
        let mut ans: Option<Box<ListNode>> = None;
        for v in l.iter().rev() {
            ans = Some(Box::new(ListNode {
                val: *v,
                next: ans
            }))
        }
        ans
    }
}
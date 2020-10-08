#Given a linked list, rotate the list to the right by k places, where k is non-negative.
#
#Example 1:
#
#Input: 1->2->3->4->5->NULL, k = 2
#Output: 4->5->1->2->3->NULL
#Explanation:
#rotate 1 steps to the right: 5->1->2->3->4->NULL
#rotate 2 steps to the right: 4->5->1->2->3->NULL
#Example 2:
#
#Input: 0->1->2->NULL, k = 4
#Output: 2->0->1->NULL
#Explanation:
#rotate 1 steps to the right: 2->0->1->NULL
#rotate 2 steps to the right: 1->2->0->NULL
#rotate 3 steps to the right: 0->1->2->NULL
#rotate 4 steps to the right: 2->0->1->NULL

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def rotateRight(self, head: ListNode, k: int) -> ListNode:
        '''if not head or k == 0:
            return head
        l = []
        while head:
            l.append(head.val)
            head = head.next
        k = k if k < len(l) else k % len(l)
        l = l[-k:] + l[:-k]
        head = ListNode(l.pop(0))
        cur = head
        while l:
            cur.next = ListNode(l.pop(0))
            cur = cur.next
        return head'''
        if not head or k == 0:
            return head
        last = head
        size = 1
        while last.next:
            size += 1
            last = last.next
        k = k if k < size else k % size
        new_last = head
        for _ in range(size - k - 1):
            new_last = new_last.next
        last.next = head
        head = new_last.next
        new_last.next = None
        return head
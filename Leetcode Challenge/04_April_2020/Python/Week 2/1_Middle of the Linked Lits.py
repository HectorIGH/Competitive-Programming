#Given a non-empty, singly linked list with head node head, return a middle node of linked list.
#
#If there are two middle nodes, return the second middle node.
#
# 
#
#Example 1:
#
#Input: [1,2,3,4,5]
#Output: Node 3 from this list (Serialization: [3,4,5])
#The returned node has value 3.  (The judge's serialization of this node is [3,4,5]).
#Note that we returned a ListNode object ans, such that:
#ans.val = 3, ans.next.val = 4, ans.next.next.val = 5, and ans.next.next.next = NULL.
#Example 2:
#
#Input: [1,2,3,4,5,6]
#Output: Node 4 from this list (Serialization: [4,5,6])
#Since the list has two middle nodes with values 3 and 4, we return the second one.
# 
#
#Note:
#
#The number of nodes in the given list will be between 1 and 100.

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def middleNode(self, head: ListNode) -> ListNode:        
        #current = head
        #ahead = head
        
        #while ahead and ahead.next:
        #    current = current.next
        #    ahead = ahead.next.next
        #return current
        
        #Second solution, convoluted
        
        #even = False
        #if head.next == None:
        #    return head
        #elif head.next.next == None:
        #    return head.next
        
        #while current.next != None:
        #    if ahead.next != None:
        #        ahead = ahead.next
        #        even = True
        #    if ahead.next !=  None:
        #        ahead = ahead.next
        #        even = False
        #    else:
        #        if even: # even size
        #            return current.next
        #        else: # odd size
        #            return current
        #    current = current.next
        
        ###Third Solution 
        
        lista = {}
        i = 0
        while head:
            lista[i] = head
            head = head.next
            i += 1
        return lista[i // 2]
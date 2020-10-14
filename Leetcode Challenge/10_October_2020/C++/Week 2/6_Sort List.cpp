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

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    ListNode* sortList(ListNode* head) {
        if(head == nullptr || head->next == nullptr)
            return head;
        ListNode* mid = getMid(head);
        ListNode* left = sortList(head);
        ListNode* right = sortList(mid);
        return merge(left, right);
    }
    ListNode* merge(ListNode* list1, ListNode* list2) {
        ListNode* dummyHead = new ListNode();
        ListNode* tail = dummyHead;
        while(list1 != nullptr && list2 != nullptr) {
            if (list1->val < list2->val) {
                tail->next = list1;
                list1 = list1->next;
                tail = tail->next;
            } else {
                tail->next = list2;
                list2 = list2->next;
                tail = tail->next;
            }
        }
        tail->next = (list1 != nullptr) ? list1 : list2;
        return dummyHead->next;
    }

    ListNode* getMid(ListNode* head) {
        ListNode* midPrev = nullptr;
        while(head != nullptr && head->next != nullptr) {
            midPrev = (midPrev == nullptr) ? head : midPrev->next;
            head = head->next->next;
        }
        ListNode* mid = midPrev->next;
        midPrev->next = nullptr;
        return mid;
    }
};
//Given a singly linked list L: L0→L1→…→Ln-1→Ln,
//reorder it to: L0→Ln→L1→Ln-1→L2→Ln-2→…
//
//You may not modify the values in the list's nodes, only nodes itself may be changed.
//
//Example 1:
//
//Given 1->2->3->4, reorder it to 1->4->2->3.
//Example 2:
//
//Given 1->2->3->4->5, reorder it to 1->5->2->4->3.

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
    void reorderList(ListNode* head) {
        //step 1: find middle
        if(head == nullptr) {
            return ;
        }
        ListNode* slow = head;
        ListNode* fast = head;
        while(fast->next != nullptr && fast->next->next != nullptr) {
            slow = slow->next;
            fast = fast->next->next;
        }
        //step 2: reverse second half
        ListNode* prev = nullptr;
        ListNode* curr = slow->next;
        ListNode* nextt = new ListNode();
        while(curr != nullptr) {
            nextt = curr->next;
            curr->next = prev;
            prev = curr;
            curr = nextt;
        }
        slow->next = nullptr;
        //step 3: merge lists
        ListNode* head1 = head;
        ListNode* head2 = prev;
        while(head2 != nullptr) {
            nextt = head1->next;
            head1->next = head2;
            head1 = head2;
            head2 = nextt;
        }
    }
};
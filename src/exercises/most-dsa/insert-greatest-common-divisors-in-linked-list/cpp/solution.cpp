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
    ListNode* insertGreatestCommonDivisors(ListNode* head) {
        if (!head) return nullptr;

        ListNode* cur = head;

        while (cur->next) {
            int n1 = cur->val, n2 = cur->next->val;
            int gcdValue = gcd(n1, n2);
            ListNode* newNode = new ListNode(gcdValue, cur->next);
            cur->next = newNode;
            cur = newNode->next;
        }

        return head;
    }

private:
    int gcd(int a, int b) {
        while (b > 0) {
            int temp = b;
            b = a % b;
            a = temp;
        }
        return a;
    }
};

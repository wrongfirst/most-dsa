class MyCircularQueue {
private:
    struct ListNode {
        int val;
        ListNode* next;
        ListNode(int v) : val(v), next(nullptr) {}
    };

    int space;
    ListNode* left;
    ListNode* right;

public:
    MyCircularQueue(int k) {
        space = k;
        left = new ListNode(0);
        right = left;
    }

    bool enQueue(int value) {
        if (isFull()) return false;

        ListNode* cur = new ListNode(value);
        if (isEmpty()) {
            left->next = cur;
            right = cur;
        } else {
            right->next = cur;
            right = cur;
        }

        space--;
        return true;
    }

    bool deQueue() {
        if (isEmpty()) return false;

        ListNode* tmp = left->next;
        left->next = left->next->next;
        delete tmp;
        if (!left->next) {
            right = left;
        }

        space++;
        return true;
    }

    int Front() {
        return isEmpty() ? -1 : left->next->val;
    }

    int Rear() {
        return isEmpty() ? -1 : right->val;
    }

    bool isEmpty() {
        return left->next == nullptr;
    }

    bool isFull() {
        return space == 0;
    }
};
